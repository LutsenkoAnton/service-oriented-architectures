mod containers;

use containers::{start_users, start_usersdb};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
struct User {
    pub username: String,
    password: String,
    pub name: String,
    pub surname: String,
    pub birthdate: String,
    pub status: String,
    pub mail: String,
    pub phone: String,
}

#[derive(Debug, Deserialize)]
struct PostResponse {
    #[allow(unused)]
    id: i32,
}

#[derive(Debug, Deserialize)]
struct PutResponse {
    pub name: String,
    pub surname: String,
    pub birthdate: String,
    pub mail: String,
    pub status: String,
    pub phone: String,
}

#[derive(Debug, Serialize)]
struct ChangeMail {
    mail: String,
}

#[tokio::test]
async fn users_create() {
    let usersdb = start_usersdb().await;
    let users = start_users(&usersdb.database_url).await;
    let user = User {
        username: "anton".to_string(),
        password: "aboba".to_string(),
        name: "anton".to_string(),
        surname: "ll".to_string(),
        birthdate: "2025-06-04".to_string(),
        status: "I'm good".to_string(),
        mail: "boba@ya.ru".to_string(),
        phone: "88005553535".to_string(),
    };
    let client = Client::new();
    let res = client
        .post(users.get_addr("user"))
        .json(&user)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert!(res.json::<PostResponse>().await.is_ok());
    // assert_eq!(res.text().json().unwrap())
    let users_table = sqlx::query!("SELECT * FROM users;")
        .fetch_all(&usersdb.pool)
        .await
        .unwrap();
    assert_eq!(users_table.len(), 1, "Database has extra rows");
    let user_row = &users_table[0];
    assert_eq!(user_row.username, user.username);
    assert_eq!(user_row.name, user.name);
    assert_eq!(user_row.surname, user.surname);
    assert_eq!(user_row.birthdate, user.birthdate);
    assert_eq!(user_row.status, user.status);
    assert_eq!(user_row.mail, user.mail);
    assert_eq!(user_row.phone, user.phone);
    let user2 = User {
        username: "anton".to_string(),
        password: "antotheraboba".to_string(),
        name: "notanton".to_string(),
        surname: "notll".to_string(),
        birthdate: "2025-06-02".to_string(),
        status: "I'm not good".to_string(),
        mail: "notboba@ya.ru".to_string(),
        phone: "88005553536".to_string(),
    };
    let res = client
        .post(users.get_addr("user"))
        .json(&user2)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 403);
    let mut user3 = user.clone();
    user3.username = "notanton".to_string();
    let res = client
        .post(users.get_addr("user"))
        .json(&user3)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert!(res.json::<PostResponse>().await.is_ok());
}

#[tokio::test]
async fn users_login() {
    let usersdb = start_usersdb().await;
    let users = start_users(&usersdb.database_url).await;
    let user = User {
        username: "anton".to_string(),
        password: "aboba".to_string(),
        name: "anton".to_string(),
        surname: "ll".to_string(),
        birthdate: "2025-06-04".to_string(),
        status: "I'm good".to_string(),
        mail: "boba@ya.ru".to_string(),
        phone: "88005553535".to_string(),
    };
    let client = Client::new();
    let res = client
        .post(users.get_addr("user"))
        .json(&user)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert!(res.json::<PostResponse>().await.is_ok());
    let res = client
        .get(users.get_addr("login"))
        .query(&[("username", &user.username), ("password", &user.password)])
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.cookies().count(), 1);
    let res = client
        .get(users.get_addr("login"))
        .query(&[
            ("username", user.username.as_str()),
            ("password", "badpassword(("),
        ])
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 403);
    let res = client
        .get(users.get_addr("login"))
        .query(&[
            ("username", "baduserame(((("),
            ("password", user.password.as_str()),
        ])
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 403);
}

#[tokio::test]
async fn users_update() {
    let usersdb = start_usersdb().await;
    let users = start_users(&usersdb.database_url).await;
    let user = User {
        username: "anton".to_string(),
        password: "aboba".to_string(),
        name: "anton".to_string(),
        surname: "ll".to_string(),
        birthdate: "2025-06-04".to_string(),
        status: "I'm good".to_string(),
        mail: "boba@ya.ru".to_string(),
        phone: "88005553535".to_string(),
    };
    let client = Client::builder().cookie_store(true).build().unwrap();
    let res = client
        .post(users.get_addr("user"))
        .json(&user)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert!(res.json::<PostResponse>().await.is_ok());
    let res = client
        .get(users.get_addr("login"))
        .query(&[("username", &user.username), ("password", &user.password)])
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let changes = ChangeMail {
        mail: "new_mail@example.com".to_string(),
    };
    let res = client
        .put(users.get_addr("user/anton"))
        .json(&changes)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let res_user = res.json::<PutResponse>().await.unwrap();
    assert_eq!(res_user.name, user.name);
    assert_eq!(res_user.surname, user.surname);
    assert_eq!(res_user.birthdate, user.birthdate);
    assert_eq!(res_user.mail, "new_mail@example.com");
    assert_eq!(res_user.phone, user.phone);
    assert_eq!(res_user.status, user.status);
    assert_eq!(
        sqlx::query!("SELECT mail FROM users;")
            .fetch_one(&usersdb.pool)
            .await
            .unwrap()
            .mail,
        "new_mail@example.com"
    );
    let res = client
        .put(users.get_addr("user/notanton"))
        .json(&changes)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 403);
}
