use axum::{extract::Query, http::StatusCode};
use axum_extra::extract::cookie::{Cookie, CookieJar};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LoginParams {
    pub username: String,
    pub password: String,
}

pub async fn login(
    mut jar: CookieJar,
    Query(params): Query<LoginParams>,
) -> Result<CookieJar, StatusCode> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    let res = client
        .get("http://users:3000/login")
        .query(&[("username", params.username), ("password", params.password)])
        .send()
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    if res.status().is_success() {
        for cookie in res.cookies() {
            jar = jar.add(Cookie::new(
                cookie.name().to_string(),
                cookie.value().to_string(),
            ));
        }
        Ok(jar)
    } else {
        Err(res.status())
    }
}
