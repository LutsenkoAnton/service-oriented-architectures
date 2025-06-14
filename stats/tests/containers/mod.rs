use std::{path::PathBuf, time::Duration};

use clickhouse::Client;
use rdkafka::{config::ClientConfig, producer::FutureProducer};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt, core::{WaitFor, IntoContainerPort}, runners::AsyncRunner,
};
use testcontainers_modules::clickhouse::ClickHouse;
use rand::Rng;

pub struct StatsBroker {
    _container: ContainerAsync<GenericImage>,
    pub broker_url: String,
    pub producer: FutureProducer,
}

pub async fn start_broker() -> StatsBroker {
    let cmd = ["/start.sh"];
    let mut rng = rand::rng();
    let random_port = rng.random_range(5050..6050);
    let container = GenericImage::new("apache/kafka", "4.0.0")
        .with_exposed_port(9997.tcp())
        .with_mapped_port(random_port, 9092.tcp())
        .with_network("testing_network")
        .with_copy_to("/start.sh", PathBuf::from("/home/anton/Documents/hse/service-oriented-architectures/stats/start.sh"))
        .with_env_var("KAFKA_LISTENERS", "CONTROLLER://localhost:9091,HOST://0.0.0.0:9092,DOCKER://0.0.0.0:9093")
        // .with_env_var("KAFKA_ADVERTISED_LISTENERS", "HOST://localhost:9092,DOCKER://localhost:9093")
        .with_env_var("KAFKA_LISTENER_SECURITY_PROTOCOL_MAP", "CONTROLLER:PLAINTEXT,DOCKER:PLAINTEXT,HOST:PLAINTEXT")
        .with_env_var("KAFKA_NODE_ID", "1")
        .with_env_var("KAFKA_PROCESS_ROLES", "broker,controller")
        .with_env_var("KAFKA_CONTROLLER_LISTENER_NAMES", "CONTROLLER")
        .with_env_var("KAFKA_CONTROLLER_QUORUM_VOTERS", "1@localhost:9091")
        .with_env_var("KAFKA_INTER_BROKER_LISTENER_NAME", "DOCKER")
        .with_env_var("KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR", "1")
        .with_env_var("KAFKA_JMX_PORT", "9997")
        .with_env_var("KAFKA_JMX_HOSTNAME", "kafka")
        .with_env_var("PLEASE_PORT", format!("{}", random_port));
    let container = container
        .with_cmd(cmd)
        .start()
        .await
        .unwrap();
    let broker_url = format!("{}:9093", container.get_bridge_ip_address().await.unwrap());
    let localhost_broker_url = format!("localhost:{}", container.get_host_port_ipv4(9092).await.unwrap());
    dbg!(&localhost_broker_url);
    let producer = ClientConfig::new()
        .set("bootstrap.servers", &localhost_broker_url)
        .set("message.timeout.ms", "5000")
        .set("delivery.timeout.ms", "5000")
        .create()
        .unwrap();
    StatsBroker {
        _container: container,
        broker_url,
        producer,
    }
}

pub struct StatsDb {
    _container: ContainerAsync<ClickHouse>,
    pub database_url: String,
    pub client: Client,
}

pub async fn start_statsdb(stats_broker: Option<&str>) -> StatsDb {
    let container = ClickHouse::default()
        .with_tag("25.5.1")
        .with_network("testing_network")
        .with_copy_to("/etc/clickhouse-server/users.d/config.xml", PathBuf::from("/home/anton/Documents/hse/service-oriented-architectures/statsdb/config.xml"))
        .with_env_var("CLICKHOUSE_USER", "user")
        .with_env_var("CLICKHOUSE_PASSWORD", "pass")
        .start()
        .await
        .unwrap();
    let client = Client::default()
        .with_url(format!(
            "http://localhost:{}",
            container.get_host_port_ipv4(8123).await.unwrap()
        ))
        .with_user("user")
        .with_password("pass");
    let database_url = format!(
        "http://{}:8123",
        container.get_bridge_ip_address().await.unwrap()
    );
    if stats_broker.is_some() {
        client.query("CREATE TABLE views_queue (post_id UInt64, creator_id UInt32, time DateTime) ENGINE = Kafka(?, 'views', 'clickhouse', 'JSONEachRow')").bind(stats_broker).execute().await.unwrap();
        client.query("CREATE TABLE views (post_id UInt64, creator_id UInt32, time DateTime) ENGINE = MergeTree ORDER BY (post_id, creator_id)").execute().await.unwrap();
        client
            .query("CREATE MATERIALIZED VIEW views_mv TO views AS SELECT * FROM views_queue")
            .execute()
            .await
            .unwrap();
        client.query("CREATE TABLE likes_queue (post_id UInt64, creator_id UInt32, time DateTime) ENGINE = Kafka(?, 'likes', 'clickhouse', 'JSONEachRow')").bind(stats_broker).execute().await.unwrap();
        client.query("CREATE TABLE likes (post_id UInt64, creator_id UInt32, time DateTime) ENGINE = MergeTree ORDER BY (post_id, creator_id)").execute().await.unwrap();
        client
            .query("CREATE MATERIALIZED VIEW likes_mv TO likes AS SELECT * FROM likes_queue")
            .execute()
            .await
            .unwrap();
        client.query("CREATE TABLE comments_queue (post_id UInt64, creator_id UInt32, comment String, time DateTime) ENGINE = Kafka(?, 'comments', 'clickhouse', 'JSONEachRow')").bind(stats_broker).execute().await.unwrap();
        client.query("CREATE TABLE comments (post_id UInt64, creator_id UInt32, comment String, time DateTime) ENGINE = MergeTree ORDER BY (post_id, creator_id)").execute().await.unwrap();
        client
            .query(
                "CREATE MATERIALIZED VIEW comments_mv TO comments AS SELECT * FROM comments_queue;",
            )
            .execute()
            .await
            .unwrap();
    } else {
        client.query("CREATE TABLE views (post_id UInt64, creator_id UInt32, time DateTime) ENGINE = MergeTree ORDER BY (post_id, creator_id)").execute().await.unwrap();
        client.query("CREATE TABLE likes (post_id UInt64, creator_id UInt32, time DateTime) ENGINE = MergeTree ORDER BY (post_id, creator_id)").execute().await.unwrap();
        client.query("CREATE TABLE comments (post_id UInt64, creator_id UInt32, comment String, time DateTime) ENGINE = MergeTree ORDER BY (post_id, creator_id)").execute().await.unwrap();
    }
    StatsDb {
        _container: container,
        database_url,
        client,
    }
}

pub struct Stats {
    _container: ContainerAsync<GenericImage>,
    pub url: String,
}

pub async fn start_stats(statsdb: &str) -> Stats {
    let container = GenericImage::new("stats-local", "latest")
        .with_exposed_port(7000.tcp())
        .with_network("testing_network")
        .with_env_var("DATABASE_URL", statsdb)
        .with_env_var("CLICKHOUSE_USER", "user")
        .with_env_var("CLICKHOUSE_PASSWORD", "pass")
        .start()
        .await
        .unwrap();
    let url = format!(
        "http://localhost:{}/",
        container.get_host_port_ipv4(7000).await.unwrap()
    );
    Stats {
        _container: container,
        url,
    }
}
