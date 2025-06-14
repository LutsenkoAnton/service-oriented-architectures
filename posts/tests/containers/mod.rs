use sqlx::{PgPool, migrate};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt, core::IntoContainerPort, runners::AsyncRunner,
};
use testcontainers_modules::postgres::Postgres;

pub struct PostsDb {
    _container: ContainerAsync<Postgres>,
    pub database_url: String,
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
    migrate!().run(&pool).await.unwrap();
    PostsDb {
        _container: container,
        database_url,
        pool,
    }
}

pub struct Posts {
    _container: ContainerAsync<GenericImage>,
    pub url: String,
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
    Posts {
        _container: container,
        url,
    }
}
