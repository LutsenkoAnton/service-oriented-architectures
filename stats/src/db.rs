use crate::Result;
use chrono::NaiveDate;
use clickhouse::{Client, Row, query::RowCursor};
use serde::Deserialize;

#[derive(Debug, Deserialize, Row)]
pub struct CountResponse {
    pub comments: Option<u64>,
    pub likes: Option<u64>,
    pub views: Option<u64>,
}

#[derive(Debug, Deserialize, Row)]
pub struct DynamicsResponse {
    pub count: u64,
    #[serde(with = "clickhouse::serde::chrono::date")]
    pub day: NaiveDate,
}

#[derive(Debug, Deserialize, Row)]
pub struct Post {
    pub post_id: i64,
}

#[derive(Debug, Deserialize, Row)]
pub struct User {
    pub user_id: i32,
}

pub async fn count_stats(client: &Client, post_id: i64) -> Result<CountResponse> {
    Ok(client
        .query("SELECT (SELECT countIf(post_id=?) FROM comments) AS comments, (SELECT countIf(post_id=?) FROM likes) AS likes, (SELECT countIf(post_id=?) FROM views) AS views")
        .bind(post_id)
        .bind(post_id)
        .bind(post_id).fetch_one().await?)
}

pub async fn dynamics_likes(client: &Client, post_id: i64) -> Result<RowCursor<DynamicsResponse>> {
    Ok(client
        .query(
            "SELECT COUNT(*) as count, day FROM likes WHERE post_id=? GROUP BY toDate(time) AS day",
        )
        .bind(post_id)
        .fetch()?)
}
pub async fn dynamics_views(client: &Client, post_id: i64) -> Result<RowCursor<DynamicsResponse>> {
    Ok(client
        .query(
            "SELECT COUNT(*) as count, day FROM views WHERE post_id=? GROUP BY toDate(time) AS day",
        )
        .bind(post_id)
        .fetch()?)
}
pub async fn dynamics_comments(
    client: &Client,
    post_id: i64,
) -> Result<RowCursor<DynamicsResponse>> {
    Ok(client
        .query(
            "SELECT COUNT(*) as count, day FROM comments WHERE post_id=? GROUP BY toDate(time) AS day",
        )
        .bind(post_id)
        .fetch()?)
}

pub async fn get_top_10_posts_likes(client: &Client) -> Result<RowCursor<Post>> {
    Ok(client
        .query("SELECT post_id FROM likes GROUP BY post_id ORDER BY COUNT(*) DESC LIMIT 10")
        .fetch()?)
}
pub async fn get_top_10_posts_views(client: &Client) -> Result<RowCursor<Post>> {
    Ok(client
        .query("SELECT post_id FROM views GROUP BY post_id ORDER BY COUNT(*) DESC LIMIT 10")
        .fetch()?)
}
pub async fn get_top_10_posts_comments(client: &Client) -> Result<RowCursor<Post>> {
    Ok(client
        .query("SELECT post_id FROM comments GROUP BY post_id ORDER BY COUNT(*) DESC LIMIT 10")
        .fetch()?)
}

pub async fn get_top_10_users_likes(client: &Client) -> Result<RowCursor<User>> {
    Ok(client
        .query("SELECT creator_id AS user_id FROM likes GROUP BY creator_id ORDER BY COUNT(*) DESC LIMIT 10")
        .fetch()?)
}
pub async fn get_top_10_users_views(client: &Client) -> Result<RowCursor<User>> {
    Ok(client
        .query("SELECT creator_id AS user_id FROM views GROUP BY creator_id ORDER BY COUNT(*) DESC LIMIT 10")
        .fetch()?)
}
pub async fn get_top_10_users_comments(client: &Client) -> Result<RowCursor<User>> {
    Ok(client
        .query("SELECT creator_id AS user_id FROM comments GROUP BY creator_id ORDER BY COUNT(*) DESC LIMIT 10")
        .fetch()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clickhouse::test;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Row)]
    struct ViewsRow {
        pub post_id: u64,
        pub creator_id: u32,
        pub time: chrono::NaiveDateTime,
    }
    #[derive(Serialize, Deserialize, Row)]
    struct LikesRow {
        pub post_id: u64,
        pub creator_id: u32,
        pub time: chrono::NaiveDateTime,
    }
    #[derive(Serialize, Deserialize, Row)]
    struct CommentsRow {
        pub post_id: u64,
        pub creator_id: u32,
        pub comment: String,
        pub time: chrono::NaiveDateTime,
    }

    async fn init_db(client: &Client) {
        client.query("CREATE TABLE views ( post_id UInt64, creator_id UInt32, time DateTime) ENGINE = MergeTree ORDER BY (post_id, creator_id)").execute().await.unwrap();
        client.query("CREATE TABLE likes ( post_id UInt64, creator_id UInt32, time DateTime) ENGINE = MergeTree ORDER BY (post_id, creator_id)").execute().await.unwrap();
        client.query("CREATE TABLE comments ( post_id UInt64, creator_id UInt32, comment String, time DateTime) ENGINE = MergeTree ORDER BY (post_id, creator_id)").execute().await.unwrap();
    }

    async fn fill_db(client: &Client) {
        let mut insert = client.insert("views").unwrap();
        let one_day = chrono::Days::new(1);
        let mut time  = chrono::NaiveDate::from_ymd_opt(2025, 5, 25).unwrap().and_hms_opt(12, 0, 0).unwrap();
        insert.write(&ViewsRow{post_id: 1, creator_id: 1, time}).await.unwrap();
        insert.write(&ViewsRow{post_id: 2, creator_id: 1, time}).await.unwrap();
        insert.write(&ViewsRow{post_id: 3, creator_id: 1, time}).await.unwrap();
        insert.write(&ViewsRow{post_id: 4, creator_id: 1, time}).await.unwrap();
        time = time + one_day;
        insert.write(&ViewsRow{post_id: 1, creator_id: 2, time}).await.unwrap();
        insert.write(&ViewsRow{post_id: 1, creator_id: 3, time}).await.unwrap();
        insert.write(&ViewsRow{post_id: 1, creator_id: 4, time}).await.unwrap();
        insert.write(&ViewsRow{post_id: 1, creator_id: 5, time}).await.unwrap();
        time = time + one_day;
        insert.write(&ViewsRow{post_id: 1, creator_id: 6, time}).await.unwrap();
        insert.end().await.unwrap();
        let mut insert = client.insert("likes").unwrap();
        let mut time  = chrono::NaiveDate::from_ymd_opt(2025, 5, 25).unwrap().and_hms_opt(12, 0, 0).unwrap();
        insert.write(&LikesRow{post_id: 1, creator_id: 1, time}).await.unwrap();
        insert.write(&LikesRow{post_id: 2, creator_id: 1, time}).await.unwrap();
        insert.write(&LikesRow{post_id: 3, creator_id: 1, time}).await.unwrap();
        insert.write(&LikesRow{post_id: 4, creator_id: 1, time}).await.unwrap();
        insert.write(&LikesRow{post_id: 1, creator_id: 2, time}).await.unwrap();
        insert.write(&LikesRow{post_id: 4, creator_id: 3, time}).await.unwrap();
        time = time + one_day;
        insert.write(&LikesRow{post_id: 1, creator_id: 4, time}).await.unwrap();
        insert.write(&LikesRow{post_id: 1, creator_id: 4, time}).await.unwrap();
        insert.write(&LikesRow{post_id: 1, creator_id: 4, time}).await.unwrap();
        insert.write(&LikesRow{post_id: 1, creator_id: 4, time}).await.unwrap();
        insert.write(&LikesRow{post_id: 2, creator_id: 5, time}).await.unwrap();
        time = time + one_day;
        insert.write(&LikesRow{post_id: 4, creator_id: 6, time}).await.unwrap();
        insert.write(&LikesRow{post_id: 2, creator_id: 5, time}).await.unwrap();
        insert.write(&LikesRow{post_id: 2, creator_id: 5, time}).await.unwrap();
        insert.write(&LikesRow{post_id: 2, creator_id: 5, time}).await.unwrap();
        insert.end().await.unwrap();
        let mut insert = client.insert("comments").unwrap();
        let mut time  = chrono::NaiveDate::from_ymd_opt(2025, 5, 25).unwrap().and_hms_opt(12, 0, 0).unwrap();
        let comment = "Comment text, does not matter".to_string();
        insert.write(&CommentsRow{post_id: 1, creator_id: 1, comment: comment.clone(), time}).await.unwrap();
        insert.write(&CommentsRow{post_id: 2, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
        insert.write(&CommentsRow{post_id: 3, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
        insert.write(&CommentsRow{post_id: 4, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
        insert.write(&CommentsRow{post_id: 5, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
        insert.write(&CommentsRow{post_id: 6, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
        time = time + one_day;
        insert.write(&CommentsRow{post_id: 5, creator_id: 6, comment: comment.clone(), time}).await.unwrap();
        insert.write(&CommentsRow{post_id: 5, creator_id: 3, comment: comment.clone(), time}).await.unwrap();
        insert.write(&CommentsRow{post_id: 5, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
        insert.write(&CommentsRow{post_id: 5, creator_id: 4, comment: comment.clone(), time}).await.unwrap();
        insert.write(&CommentsRow{post_id: 5, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
        insert.write(&CommentsRow{post_id: 5, creator_id: 2, comment: comment.clone(), time}).await.unwrap();
        insert.end().await.unwrap();
    }

    #[tokio::test]
    async fn test_count_stats() {
        let mock = test::Mock::new();
        let client = Client::default().with_url(mock.url());
        init_db(&client).await;
        fill_db(&client).await;
        let stats = count_stats(&client, 1).await.unwrap();
        assert_eq!(stats.views, Some(6));
        assert_eq!(stats.likes, Some(6));
        assert_eq!(stats.comments, Some(1));
    }

}
