use crate::state::AppState;
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use axum_extra::extract::cookie::CookieJar;
use reqwest::{Client, Url, cookie::Jar};
use std::sync::Arc;

pub async fn user_get(
    jar: CookieJar,
    Path(username): Path<String>,
    State(app_state): State<AppState>,
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
        .get(format!("{}/user/{}", &app_state.users_url, username))
        .send()
        .await;
    match res {
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap(),
        Ok(res) => {
            let status = res.status();
            if !status.is_success() {
                return Response::builder()
                    .status(status)
                    .body(Body::empty())
                    .unwrap();
            }
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
