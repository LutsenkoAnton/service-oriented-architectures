mod containers;
mod fill_db;

use chrono::{Days, NaiveDate};
use containers::{start_stats, start_statsdb};
use fill_db::fill_db;
use stats_client::stats_client::*;
use stats_client::*;

pub mod stats_client {
    tonic::include_proto!("stats");
}

#[tokio::test]
async fn stats_activity() {
    let mut statsdb = start_statsdb(None).await;
    let stats = start_stats(&statsdb.database_url).await;
    let mut grpc_client = StatsClient::connect(stats.url).await.unwrap();
    fill_db(&mut statsdb.client).await;
    let res = grpc_client
        .count_activity(CountActivityRequest { post_id: 2 })
        .await
        .unwrap()
        .into_inner();
    assert_eq!(res.comments, 16);
    assert_eq!(res.likes, 22);
    assert_eq!(res.views, 13);
    let res = grpc_client
        .count_activity(CountActivityRequest { post_id: 200 })
        .await
        .unwrap()
        .into_inner();
    assert_eq!(res.comments, 0);
    assert_eq!(res.likes, 0);
    assert_eq!(res.views, 0);
}

#[tokio::test]
async fn stats_dynamics() {
    let mut statsdb = start_statsdb(None).await;
    let stats = start_stats(&statsdb.database_url).await;
    let mut grpc_client = StatsClient::connect(stats.url).await.unwrap();
    fill_db(&mut statsdb.client).await;
    let mut stream = grpc_client
        .dynamics(DynamicsRequest {
            post_id: 3,
            r#type: ActivityType::Views.into(),
        })
        .await
        .unwrap()
        .into_inner();
    let mut time = NaiveDate::from_ymd_opt(2025, 6, 4)
        .unwrap();
    time = time + Days::new(1);
    let dynamic = stream.message().await.unwrap().unwrap();
    assert_eq!(dynamic.count, 2);
    assert_eq!(dynamic.day, time.to_string());
    time = time + Days::new(1);
    let dynamic = stream.message().await.unwrap().unwrap();
    assert_eq!(dynamic.count, 4);
    assert_eq!(dynamic.day, time.to_string());
    time = time + Days::new(1);
    let dynamic = stream.message().await.unwrap().unwrap();
    assert_eq!(dynamic.count, 1);
    assert_eq!(dynamic.day, time.to_string());
    time = time + Days::new(1);
    let dynamic = stream.message().await.unwrap().unwrap();
    assert_eq!(dynamic.count, 2);
    assert_eq!(dynamic.day, time.to_string());
    time = time + Days::new(1);
    let dynamic = stream.message().await.unwrap().unwrap();
    assert_eq!(dynamic.count, 1);
    assert_eq!(dynamic.day, time.to_string());
    time = time + Days::new(1);
    let dynamic = stream.message().await.unwrap().unwrap();
    assert_eq!(dynamic.count, 2);
    assert_eq!(dynamic.day, time.to_string());
    time = time + Days::new(1);
    time = time + Days::new(1);
    let dynamic = stream.message().await.unwrap().unwrap();
    assert_eq!(dynamic.count, 1);
    assert_eq!(dynamic.day, time.to_string());
    time = time + Days::new(1);
    let dynamic = stream.message().await.unwrap().unwrap();
    assert_eq!(dynamic.count, 1);
    assert_eq!(dynamic.day, time.to_string());
    time = time + Days::new(1);
    let dynamic = stream.message().await.unwrap().unwrap();
    assert_eq!(dynamic.count, 3);
    assert_eq!(dynamic.day, time.to_string());
    assert!(stream.message().await.unwrap().is_none());
    let mut stream = grpc_client
        .dynamics(DynamicsRequest {
            post_id: 300,
            r#type: ActivityType::Views.into(),
        })
        .await
        .unwrap()
        .into_inner();
    assert!(stream.message().await.unwrap().is_none());
}
#[tokio::test]
async fn stats_top10() {
    let mut statsdb = start_statsdb(None).await;
    let stats = start_stats(&statsdb.database_url).await;
    let mut grpc_client = StatsClient::connect(stats.url).await.unwrap();
    fill_db(&mut statsdb.client).await;
    let mut stream = grpc_client
        .get_top10_users(GetTop10UsersRequest{
            r#type: ActivityType::Comments.into(),
        })
        .await
        .unwrap()
        .into_inner();
    let user = stream.message().await.unwrap().unwrap();
    assert_eq!(user.user_id, 5);
    let user = stream.message().await.unwrap().unwrap();
    assert_eq!(user.user_id, 6);
    let user = stream.message().await.unwrap().unwrap();
    assert_eq!(user.user_id, 4);
    let user = stream.message().await.unwrap().unwrap();
    assert_eq!(user.user_id, 1);
    let user = stream.message().await.unwrap().unwrap();
    assert_eq!(user.user_id, 2);
    let user = stream.message().await.unwrap().unwrap();
    assert_eq!(user.user_id, 3);
    assert!(stream.message().await.unwrap().is_none());
}
