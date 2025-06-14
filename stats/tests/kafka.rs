mod containers;
mod fill_db;

use chrono::{DateTime, NaiveDate, Utc};
use clickhouse::Row;
use containers::{start_broker, start_statsdb};
use rdkafka::producer::FutureRecord;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize)]
struct View {
    post_id: i64,
    creator_id: i32,
    time: DateTime<Utc>,
}

#[derive(Debug, Row, Deserialize)]
struct RowView {
    post_id: i64,
    creator_id: i32,
    #[serde(with = "clickhouse::serde::chrono::datetime")]
    time: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct Comment {
    post_id: i64,
    creator_id: i32,
    time: DateTime<Utc>,
    comment: String,
}

#[derive(Debug, Row, Deserialize)]
struct RowComment {
    post_id: i64,
    creator_id: i32,
    comment: String,
    #[serde(with = "clickhouse::serde::chrono::datetime")]
    time: DateTime<Utc>,
}

#[tokio::test]
async fn kafka_views() {
    let stats_broker = start_broker().await;
    let statsdb = start_statsdb(Some(&stats_broker.broker_url)).await;
    let time = NaiveDate::from_ymd_opt(2025, 06, 04)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap()
        .and_utc();
    let view = View {
        post_id: 123,
        creator_id: 456,
        time,
    };
    let str_view = serde_json::to_string(&view).unwrap();
    let mut response = None;
    while response.is_none() {
        response = stats_broker.producer.send(
            FutureRecord::<String, String>::to("views").payload(&str_view),
            Duration::from_secs(0),
        ).await.ok();
    }
    tokio::time::sleep(Duration::from_secs(10)).await;
    let res: Vec<RowView> = statsdb
        .client
        .query("SELECT * FROM views")
        .fetch_all()
        .await
        .unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].post_id, 123);
    assert_eq!(res[0].creator_id, 456);
    assert_eq!(res[0].time, time);
}

#[tokio::test]
async fn kafka_comments() {
    let stats_broker = start_broker().await;
    let statsdb = start_statsdb(Some(&stats_broker.broker_url)).await;
    let time = NaiveDate::from_ymd_opt(2025, 06, 04)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap()
        .and_utc();
    let comment = Comment {
        post_id: 5,
        creator_id: 6,
        comment: "Bad post(((".to_string(),
        time: time + chrono::Days::new(1),
    };
    let str_comment = serde_json::to_string(&comment).unwrap();
    let mut response = None;
    while response.is_none() {
        response = stats_broker.producer.send(
            FutureRecord::<String, String>::to("comments").payload(&str_comment),
            Duration::from_secs(0),
        ).await.ok();
    }
    let comment = Comment {
        post_id: 3,
        creator_id: 6,
        comment: "What a good post!".to_string(),
        time,
    };
    let str_comment = serde_json::to_string(&comment).unwrap();
    let mut response = None;
    while response.is_none() {
        response = stats_broker.producer.send(
            FutureRecord::<String, String>::to("comments").payload(&str_comment),
            Duration::from_secs(0),
        ).await.ok();
    }
    tokio::time::sleep(Duration::from_secs(10)).await;
    let res: Vec<RowComment> = statsdb
        .client
        .query("SELECT * FROM comments")
        .fetch_all()
        .await
        .unwrap();
    assert_eq!(res.len(), 2);
    assert_eq!(res[0].post_id, 3);
    assert_eq!(res[0].creator_id, 6);
    assert_eq!(res[0].time, time);
    assert_eq!(res[0].comment, "What a good post!");
    assert_eq!(res[1].post_id, 5);
    assert_eq!(res[1].creator_id, 6);
    assert_eq!(res[1].time, time + chrono::Days::new(1));
    assert_eq!(res[1].comment, "Bad post(((".to_string());
}
