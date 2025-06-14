use sqlx::{PgPool, migrate};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt, core::IntoContainerPort, runners::AsyncRunner,
};
use testcontainers_modules::postgres::Postgres;

pub struct UsersDb {
    _container: ContainerAsync<Postgres>,
    pub database_url: String,
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
    migrate!().run(&pool).await.unwrap();
    UsersDb {
        _container: container,
        database_url,
        pool,
    }
}

pub struct Users {
    _container: ContainerAsync<GenericImage>,
    pub url: String,
}

impl Users {
    pub fn get_addr(&self, tail: &str) -> String {
        let mut result = self.url.clone();
        result.push_str(tail);
        result
    }
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
    Users {
        _container: container,
        url,
    }
}
