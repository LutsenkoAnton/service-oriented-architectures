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
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Row)]
    struct CountStatsRow {
        pub comments_count: Option<u64>,
        pub likes_count: Option<u64>,
        pub views_count: Option<u64>,
    }
    #[tokio::test]
    async fn test_count_stats() {
        let mock = test::Mock::new();
        let client = Client::default().with_url(mock.url());
        // init_db(&client).await;
        // fill_db(&client).await;
        mock.add(test::handlers::provide(&[CountStatsRow {
            likes_count: Some(6),
            views_count: Some(6),
            comments_count: Some(1),
        }]));
        let stats = count_stats(&client, 1).await.unwrap();
        assert_eq!(stats.views, Some(6));
        assert_eq!(stats.likes, Some(6));
        assert_eq!(stats.comments, Some(1));
    }
    #[derive(Serialize, Deserialize, Row)]
    struct DynamicsRow {
        pub count: u64,
        #[serde(with = "clickhouse::serde::chrono::date")]
        pub day: chrono::NaiveDate,
    }
    #[tokio::test]
    async fn test_dynamic_likes() {
        let mock = test::Mock::new();
        let client = Client::default().with_url(mock.url());
        let day = chrono::NaiveDate::from_ymd_opt(2025, 5, 25).unwrap();
        mock.add(test::handlers::provide(&[
            DynamicsRow { count: 2, day },
            DynamicsRow {
                count: 1,
                day: day + chrono::Days::new(1),
            },
            DynamicsRow {
                count: 3,
                day: day + chrono::Days::new(2),
            },
        ]));
        let mut stats = dynamics_likes(&client, 1).await.unwrap();
        let dynamic = stats.next().await.unwrap().unwrap();
        assert_eq!(dynamic.count, 2);
        assert_eq!(dynamic.day, day);
        let dynamic = stats.next().await.unwrap().unwrap();
        assert_eq!(dynamic.count, 1);
        assert_eq!(dynamic.day, day + chrono::Days::new(1));
        let dynamic = stats.next().await.unwrap().unwrap();
        assert_eq!(dynamic.count, 3);
        assert_eq!(dynamic.day, day + chrono::Days::new(2));
        assert!(stats.next().await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_dynamic_views() {
        let mock = test::Mock::new();
        let client = Client::default().with_url(mock.url());
        let day = chrono::NaiveDate::from_ymd_opt(2025, 5, 25).unwrap();
        mock.add(test::handlers::provide(&[
            DynamicsRow { count: 2, day },
            DynamicsRow {
                count: 1,
                day: day + chrono::Days::new(1),
            },
            DynamicsRow {
                count: 3,
                day: day + chrono::Days::new(2),
            },
        ]));
        let mut stats = dynamics_views(&client, 1).await.unwrap();
        let dynamic = stats.next().await.unwrap().unwrap();
        assert_eq!(dynamic.count, 2);
        assert_eq!(dynamic.day, day);
        let dynamic = stats.next().await.unwrap().unwrap();
        assert_eq!(dynamic.count, 1);
        assert_eq!(dynamic.day, day + chrono::Days::new(1));
        let dynamic = stats.next().await.unwrap().unwrap();
        assert_eq!(dynamic.count, 3);
        assert_eq!(dynamic.day, day + chrono::Days::new(2));
        assert!(stats.next().await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_dynamic_comments() {
        let mock = test::Mock::new();
        let client = Client::default().with_url(mock.url());
        let day = chrono::NaiveDate::from_ymd_opt(2025, 5, 25).unwrap();
        mock.add(test::handlers::provide(&[
            DynamicsRow { count: 2, day },
            DynamicsRow {
                count: 1,
                day: day + chrono::Days::new(1),
            },
            DynamicsRow {
                count: 3,
                day: day + chrono::Days::new(2),
            },
        ]));
        let mut stats = dynamics_comments(&client, 1).await.unwrap();
        let dynamic = stats.next().await.unwrap().unwrap();
        assert_eq!(dynamic.count, 2);
        assert_eq!(dynamic.day, day);
        let dynamic = stats.next().await.unwrap().unwrap();
        assert_eq!(dynamic.count, 1);
        assert_eq!(dynamic.day, day + chrono::Days::new(1));
        let dynamic = stats.next().await.unwrap().unwrap();
        assert_eq!(dynamic.count, 3);
        assert_eq!(dynamic.day, day + chrono::Days::new(2));
        assert!(stats.next().await.unwrap().is_none());
    }

    #[derive(Serialize, Deserialize, Row)]
    pub struct PostRow {
        pub post_id: i64,
    }

    #[tokio::test]
    async fn test_top_10_posts_likes() {
        let mock = test::Mock::new();
        let client = Client::default().with_url(mock.url());
        mock.add(test::handlers::provide(&[
            PostRow { post_id: 1 },
            PostRow { post_id: 6 },
            PostRow { post_id: 2 },
            PostRow { post_id: 4 },
            PostRow { post_id: 5 },
            PostRow { post_id: 3 },
            PostRow { post_id: 7 },
        ]));
        let mut stats = get_top_10_posts_likes(&client).await.unwrap();
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 1);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 6);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 2);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 4);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 5);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 3);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 7);
        let post = stats.next().await.unwrap();
        assert!(post.is_none());
    }

    #[tokio::test]
    async fn test_top_10_posts_views() {
        let mock = test::Mock::new();
        let client = Client::default().with_url(mock.url());
        mock.add(test::handlers::provide(&[
            PostRow { post_id: 1 },
            PostRow { post_id: 6 },
            PostRow { post_id: 2 },
            PostRow { post_id: 4 },
            PostRow { post_id: 5 },
            PostRow { post_id: 3 },
            PostRow { post_id: 7 },
        ]));
        let mut stats = get_top_10_posts_views(&client).await.unwrap();
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 1);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 6);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 2);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 4);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 5);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 3);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 7);
        let post = stats.next().await.unwrap();
        assert!(post.is_none());
    }

    #[tokio::test]
    async fn test_top_10_posts_comments() {
        let mock = test::Mock::new();
        let client = Client::default().with_url(mock.url());
        mock.add(test::handlers::provide(&[
            PostRow { post_id: 1 },
            PostRow { post_id: 6 },
            PostRow { post_id: 2 },
            PostRow { post_id: 4 },
            PostRow { post_id: 5 },
            PostRow { post_id: 3 },
            PostRow { post_id: 7 },
        ]));
        let mut stats = get_top_10_posts_comments(&client).await.unwrap();
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 1);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 6);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 2);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 4);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 5);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 3);
        let post = stats.next().await.unwrap().unwrap();
        assert_eq!(post.post_id, 7);
        let post = stats.next().await.unwrap();
        assert!(post.is_none());
    }

    #[derive(Serialize, Deserialize, Row)]
    pub struct UserRow {
        pub user_id: i32,
    }

    #[tokio::test]
    async fn test_top_10_users_likes() {
        let mock = test::Mock::new();
        let client = Client::default().with_url(mock.url());
        mock.add(test::handlers::provide(&[
            UserRow { user_id: 1 },
            UserRow { user_id: 6 },
            UserRow { user_id: 2 },
            UserRow { user_id: 4 },
            UserRow { user_id: 5 },
            UserRow { user_id: 3 },
            UserRow { user_id: 7 },
        ]));
        let mut stats = get_top_10_users_likes(&client).await.unwrap();
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 1);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 6);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 2);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 4);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 5);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 3);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 7);
        let user = stats.next().await.unwrap();
        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_top_10_users_views() {
        let mock = test::Mock::new();
        let client = Client::default().with_url(mock.url());
        mock.add(test::handlers::provide(&[
            UserRow { user_id: 1 },
            UserRow { user_id: 6 },
            UserRow { user_id: 2 },
            UserRow { user_id: 4 },
            UserRow { user_id: 5 },
            UserRow { user_id: 3 },
            UserRow { user_id: 7 },
        ]));
        let mut stats = get_top_10_users_views(&client).await.unwrap();
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 1);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 6);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 2);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 4);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 5);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 3);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 7);
        let user = stats.next().await.unwrap();
        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_top_10_users_comments() {
        let mock = test::Mock::new();
        let client = Client::default().with_url(mock.url());
        mock.add(test::handlers::provide(&[
            UserRow { user_id: 1 },
            UserRow { user_id: 6 },
            UserRow { user_id: 2 },
            UserRow { user_id: 4 },
            UserRow { user_id: 5 },
            UserRow { user_id: 3 },
            UserRow { user_id: 7 },
        ]));
        let mut stats = get_top_10_users_comments(&client).await.unwrap();
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 1);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 6);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 2);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 4);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 5);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 3);
        let user = stats.next().await.unwrap().unwrap();
        assert_eq!(user.user_id, 7);
        let user = stats.next().await.unwrap();
        assert!(user.is_none());
    }
}
