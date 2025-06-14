use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use axum_extra::extract::cookie::CookieJar;
use reqwest::{Client, Url, cookie::Jar};
use std::sync::Arc;

use crate::state::AppState;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    name: Option<String>,
    surname: Option<String>,
    birthdate: Option<String>,
    status: Option<String>,
    mail: Option<String>,
    phone: Option<String>,
}

pub async fn user_put(
    jar: CookieJar,
    Path(username): Path<String>,
    State(app_state): State<AppState>,
    Json(user): Json<User>,
) -> Response {
    let session = jar
        .get("session")
        .map(|cookie| cookie.to_string())
        .unwrap_or(String::new());
    let jar = Jar::default();
    jar.add_cookie_str(&session, &"http://users".parse::<Url>().unwrap());

    let client = Client::builder()
        .cookie_provider(Arc::new(jar))
        .build()
        .unwrap();
    let res = client
        .put(format!("{}/user/{}", &app_state.users_url, username))
        .json(&user)
        .send()
        .await;
    match res {
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap(),
        Ok(res) => {
            let status = res.status();
            match res.text().await {
                Err(_) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::empty())
                    .unwrap(),
                Ok(text) => Response::builder()
                    .status(status)
                    .body(Body::from(text))
                    .unwrap(),
            }
        }
    }
}
