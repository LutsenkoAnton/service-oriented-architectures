use chrono::{Days, NaiveDate, DateTime, Utc};
use clickhouse::{Client, Row};
use serde::Serialize;

#[derive(Row, Debug, Serialize)]
struct View {
    pub post_id: i64,
    pub creator_id: i32,
    #[serde(with = "clickhouse::serde::chrono::datetime")]
    pub time: DateTime<Utc>,
}

#[derive(Row, Debug, Serialize)]
struct Like {
    pub post_id: i64,
    pub creator_id: i32,
    #[serde(with = "clickhouse::serde::chrono::datetime")]
    pub time: DateTime<Utc>,
}

#[derive(Row, Debug, Serialize)]
struct Comment {
    pub post_id: i64,
    pub creator_id: i32,
    pub comment: String,
    #[serde(with = "clickhouse::serde::chrono::datetime")]
    pub time: DateTime<Utc>,
}

pub async fn fill_db(client: &mut Client) {
    fill_db_comments(client).await;
    fill_db_likes(client).await;
    fill_db_views(client).await;
}

async fn fill_db_comments(client: &mut Client) {
    let mut time = NaiveDate::from_ymd_opt(2025, 6, 4)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap()
        .and_utc();
    let mut ins = client.insert("comments").unwrap();
    let comment = "comment".to_string();
    ins.write(&Comment {post_id: 1, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Comment {post_id: 2, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 6, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 5, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 5, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Comment {post_id: 2, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 2, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Comment {post_id: 6, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Comment {post_id: 5, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 6, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 6, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 2, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 5, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 6, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 6, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 6, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 6, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Comment {post_id: 3, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 5, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 6, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 6, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Comment {post_id: 5, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 5, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 5, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 5, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 2, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 5, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 2, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 2, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Comment {post_id: 3, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 5, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 5, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 5, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 2, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 2, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 2, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 5, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Comment {post_id: 4, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 5, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 2, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 6, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Comment {post_id: 3, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 2, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 1, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 6, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 2, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Comment {post_id: 1, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 4, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 2, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 2, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 5, creator_id: 5, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 3, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
    ins.write(&Comment {post_id: 2, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
    ins.end().await.unwrap();
}
async fn fill_db_likes(client: &mut Client) {
    let mut time = NaiveDate::from_ymd_opt(2025, 6, 4)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap()
        .and_utc();
    let mut ins = client.insert("likes").unwrap();
    ins.write(&Like {post_id: 1, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 3, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 6, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Like {post_id: 5, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 4, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 6, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 3, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 4, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 1, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Like {post_id: 4, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 6, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 3, creator_id: 3, time}).await.unwrap();
    ins.write(&Like {post_id: 4, creator_id: 3, time}).await.unwrap();
    ins.write(&Like {post_id: 6, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 3, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 3, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 1, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Like {post_id: 2, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 3, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 3, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 6, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Like {post_id: 5, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 3, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 3, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 4, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 6, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 4, creator_id: 6, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Like {post_id: 4, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 6, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 3, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 4, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 3, creator_id: 5, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Like {post_id: 5, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 6, creator_id: 4, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Like {post_id: 5, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 6, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 3, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 4, creator_id: 5, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Like {post_id: 1, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 6, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 6, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 6, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Like {post_id: 3, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 6, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 3, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 6, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 1, time}).await.unwrap();
    ins.write(&Like {post_id: 3, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 3, creator_id: 5, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&Like {post_id: 2, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 2, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 3, time}).await.unwrap();
    ins.write(&Like {post_id: 4, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 4, creator_id: 2, time}).await.unwrap();
    ins.write(&Like {post_id: 5, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 6, creator_id: 5, time}).await.unwrap();
    ins.write(&Like {post_id: 3, creator_id: 4, time}).await.unwrap();
    ins.write(&Like {post_id: 1, creator_id: 3, time}).await.unwrap();
    ins.write(&Like {post_id: 3, creator_id: 6, time}).await.unwrap();
    ins.end().await.unwrap();
}

async fn fill_db_views(client: &mut Client) {
    let mut time = NaiveDate::from_ymd_opt(2025, 6, 4)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap()
        .and_utc();
    let mut ins = client.insert("views").unwrap();
    ins.write(&View {post_id: 1, creator_id: 1, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 4, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 6, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&View {post_id: 5, creator_id: 4, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 4, time}).await.unwrap();
    ins.write(&View {post_id: 2, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 6, creator_id: 1, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 1, time}).await.unwrap();
    ins.write(&View {post_id: 5, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 4, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&View {post_id: 6, creator_id: 1, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 6, creator_id: 4, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 1, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 5, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 2, creator_id: 3, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&View {post_id: 4, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 5, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 2, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 5, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 6, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 6, creator_id: 1, time}).await.unwrap();
    ins.write(&View {post_id: 2, creator_id: 3, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&View {post_id: 5, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 1, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 4, time}).await.unwrap();
    ins.write(&View {post_id: 5, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 4, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 4, time}).await.unwrap();
    ins.write(&View {post_id: 6, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 6, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 6, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&View {post_id: 5, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 2, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 2, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 2, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 4, time}).await.unwrap();
    ins.write(&View {post_id: 6, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 2, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&View {post_id: 6, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 6, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 5, creator_id: 1, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 4, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 1, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 1, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&View {post_id: 5, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 2, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 2, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 2, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 5, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 5, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 5, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&View {post_id: 2, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 6, creator_id: 1, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 2, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 4, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 1, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&View {post_id: 3, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 5, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 5, creator_id: 4, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 2, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 6, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 1, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 6, time}).await.unwrap();
    ins.write(&View {post_id: 5, creator_id: 1, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 1, time}).await.unwrap();
    time = time + Days::new(1);
    ins.write(&View {post_id: 1, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 4, time}).await.unwrap();
    ins.write(&View {post_id: 1, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 5, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 5, creator_id: 3, time}).await.unwrap();
    ins.write(&View {post_id: 4, creator_id: 2, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 4, time}).await.unwrap();
    ins.write(&View {post_id: 6, creator_id: 1, time}).await.unwrap();
    ins.write(&View {post_id: 3, creator_id: 5, time}).await.unwrap();
    ins.write(&View {post_id: 6, creator_id: 4, time}).await.unwrap();
    ins.end().await.unwrap();
}
