use clickhouse::Client;
use rand::Rng;
use rdkafka::{ClientConfig, producer::FutureProducer};
use sqlx::{PgPool, migrate};
use std::path::PathBuf;
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt, core::IntoContainerPort, runners::AsyncRunner,
};
use testcontainers_modules::{clickhouse::ClickHouse, postgres::Postgres};

pub struct Gateway {
    #[allow(unused)]
    container: ContainerAsync<GenericImage>,
    #[allow(unused)]
    usersdb: UsersDb,
    #[allow(unused)]
    users: Users,
    #[allow(unused)]
    postsdb: PostsDb,
    #[allow(unused)]
    posts: Posts,
    #[allow(unused)]
    stats_broker: StatsBroker,
    #[allow(unused)]
    statsdb: StatsDb,
    #[allow(unused)]
    stats: Stats,
    pub url: String,
}

impl Gateway {
    pub fn get_addr(&self, tail: &str) -> String {
        let mut result = self.url.clone();
        result.push_str(tail);
        result
    }
}

pub async fn start_gateway() -> Gateway {
    let usersdb = start_usersdb().await;
    let users = start_users(&usersdb.database_url).await;
    let postsdb = start_postsdb().await;
    let posts = start_posts(&postsdb.database_url).await;
    let stats_broker = start_broker().await;
    let statsdb = start_statsdb(Some(&stats_broker.broker_url)).await;
    let stats = start_stats(&statsdb.database_url).await;
    let container = GenericImage::new("service-oriented-architectures-gateway", "latest")
        .with_exposed_port(1000.tcp())
        .with_network("testing_network")
        .with_env_var("SECRET", "aboba")
        .with_env_var("USERS_URL",  &users.docker_url)
        .with_env_var("POSTS_URL",  &posts.docker_url)
        .with_env_var("STATS_URL",  &stats.docker_url)
        .with_env_var("BROKERS",  &stats_broker.broker_url)
        .start()
        .await
        .unwrap();
    let url = format!(
        "http://localhost:{}/",
        container.get_host_port_ipv4(1000).await.unwrap()
    );
    Gateway {
        container,
        usersdb,
        users,
        postsdb,
        posts,
        stats_broker,
        statsdb,
        stats,
        url,
    }
}

pub struct UsersDb {
    _container: ContainerAsync<Postgres>,
    pub database_url: String,
    #[allow(unused)]
    pub pool: PgPool,
}

pub async fn start_usersdb() -> UsersDb {
    let container = Postgres::default()
        .with_network("testing_network")
        .start()
        .await
        .unwrap();
    let pool = PgPool::connect(&format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        container.get_host_port_ipv4(5432).await.unwrap()
    ))
    .await
    .unwrap();
    let database_url = format!(
        "postgres://postgres:postgres@{}:5432/postgres",
        container.get_bridge_ip_address().await.unwrap()
    );
    migrate!("./migrations-users").run(&pool).await.unwrap();
    UsersDb {
        _container: container,
        database_url,
        pool,
    }
}

pub struct Users {
    _container: ContainerAsync<GenericImage>,
    #[allow(unused)]
    pub url: String,
    pub docker_url: String,
}

pub async fn start_users(userdb: &str) -> Users {
    let container = GenericImage::new("service-oriented-architectures-users", "latest")
        .with_exposed_port(3000.tcp())
        .with_network("testing_network")
        .with_env_var("DATABASE_URL", userdb)
        .with_env_var("SECRET", "aboba")
        .start()
        .await
        .unwrap();
    let url = format!(
        "http://localhost:{}/",
        container.get_host_port_ipv4(3000).await.unwrap()
    );
    let docker_url = format!(
        "http://{}:3000",
        container.get_bridge_ip_address().await.unwrap()
    );
    Users {
        _container: container,
        docker_url,
        url,
    }
}

pub struct PostsDb {
    _container: ContainerAsync<Postgres>,
    pub database_url: String,
    #[allow(unused)]
    pub pool: PgPool,
}

pub async fn start_postsdb() -> PostsDb {
    let container = Postgres::default()
        .with_network("testing_network")
        .start()
        .await
        .unwrap();
    let pool = PgPool::connect(&format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        container.get_host_port_ipv4(5432).await.unwrap()
    ))
    .await
    .unwrap();
    let database_url = format!(
        "postgres://postgres:postgres@{}:5432/postgres",
        container.get_bridge_ip_address().await.unwrap()
    );
    migrate!("./migrations-posts").run(&pool).await.unwrap();
    PostsDb {
        _container: container,
        database_url,
        pool,
    }
}

pub struct Posts {
    _container: ContainerAsync<GenericImage>,
    #[allow(unused)]
    pub url: String,
    pub docker_url: String,
}

pub async fn start_posts(postsdb: &str) -> Posts {
    let container = GenericImage::new("service-oriented-architectures-posts", "latest")
        .with_exposed_port(5000.tcp())
        .with_network("testing_network")
        .with_env_var("DATABASE_URL", postsdb)
        .start()
        .await
        .unwrap();
    let url = format!(
        "http://localhost:{}/",
        container.get_host_port_ipv4(5000).await.unwrap()
    );
    let docker_url = format!(
        "http://{}:5000",
        container.get_bridge_ip_address().await.unwrap()
    );
    Posts {
        _container: container,
        url,
        docker_url,
    }
}

pub struct StatsBroker {
    _container: ContainerAsync<GenericImage>,
    pub broker_url: String,
    #[allow(unused)]
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
        .with_copy_to(
            "/start.sh",
            PathBuf::from(
                "/home/anton/Documents/hse/service-oriented-architectures/stats/start.sh",
            ),
        )
        .with_env_var(
            "KAFKA_LISTENERS",
            "CONTROLLER://localhost:9091,HOST://0.0.0.0:9092,DOCKER://0.0.0.0:9093",
        )
        // .with_env_var("KAFKA_ADVERTISED_LISTENERS", "HOST://localhost:9092,DOCKER://localhost:9093")
        .with_env_var(
            "KAFKA_LISTENER_SECURITY_PROTOCOL_MAP",
            "CONTROLLER:PLAINTEXT,DOCKER:PLAINTEXT,HOST:PLAINTEXT",
        )
        .with_env_var("KAFKA_NODE_ID", "1")
        .with_env_var("KAFKA_PROCESS_ROLES", "broker,controller")
        .with_env_var("KAFKA_CONTROLLER_LISTENER_NAMES", "CONTROLLER")
        .with_env_var("KAFKA_CONTROLLER_QUORUM_VOTERS", "1@localhost:9091")
        .with_env_var("KAFKA_INTER_BROKER_LISTENER_NAME", "DOCKER")
        .with_env_var("KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR", "1")
        .with_env_var("KAFKA_JMX_PORT", "9997")
        .with_env_var("KAFKA_JMX_HOSTNAME", "kafka")
        .with_env_var("PLEASE_PORT", format!("{}", random_port));
    let container = container.with_cmd(cmd).start().await.unwrap();
    let broker_url = format!("{}:9093", container.get_bridge_ip_address().await.unwrap());
    let localhost_broker_url = format!(
        "localhost:{}",
        container.get_host_port_ipv4(9092).await.unwrap()
    );
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
    #[allow(unused)]
    pub client: Client,
}

pub async fn start_statsdb(stats_broker: Option<&str>) -> StatsDb {
    let container = ClickHouse::default()
        .with_tag("25.5.1")
        .with_network("testing_network")
        .with_copy_to(
            "/etc/clickhouse-server/users.d/config.xml",
            PathBuf::from(
                "/home/anton/Documents/hse/service-oriented-architectures/statsdb/config.xml",
            ),
        )
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
    #[allow(unused)]
    pub url: String,
    pub docker_url: String,
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
    let docker_url = format!(
        "http://{}:7000",
        container.get_bridge_ip_address().await.unwrap()
    );
    Stats {
        _container: container,
        url,
        docker_url,
    }
}
