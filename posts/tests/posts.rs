mod containers;

use containers::{start_posts, start_postsdb};
use posts_client::posts_server_client::*;
use posts_client::*;
use prost_types::FieldMask;
// use serde::{Deserialize, Serialize};
// use chrono::NaiveDateTime;

pub mod posts_client {
    tonic::include_proto!("posts");
}

async fn fill_db(client: &mut PostsServerClient<tonic::transport::Channel>) -> Vec<PostId> {
    let mut result = vec![];
    let res = client
        .create_post(CreatePostRequest {
            name: "name".to_string(),
            description: "description".to_string(),
            creator_id: 1,
            is_private: false,
            tags: vec!["tags".to_string()],
        })
        .await;
    result.push(res.unwrap().into_inner());
    let res = client
        .create_post(CreatePostRequest {
            name: "name".to_string(),
            description: "description".to_string(),
            creator_id: 1,
            is_private: true,
            tags: vec!["tags".to_string()],
        })
        .await;
    result.push(res.unwrap().into_inner());
    let res = client
        .create_post(CreatePostRequest {
            name: "name".to_string(),
            description: "description".to_string(),
            creator_id: 2,
            is_private: false,
            tags: vec!["tags".to_string()],
        })
        .await;
    result.push(res.unwrap().into_inner());
    let res = client
        .create_post(CreatePostRequest {
            name: "name".to_string(),
            description: "description".to_string(),
            creator_id: 2,
            is_private: true,
            tags: vec!["tags".to_string()],
        })
        .await;
    result.push(res.unwrap().into_inner());
    result
}

#[tokio::test]
async fn posts_create() {
    let postsdb = start_postsdb().await;
    let posts = start_posts(&postsdb.database_url).await;
    let mut grpc_client = PostsServerClient::connect(posts.url).await.unwrap();
    let res = grpc_client
        .create_post(CreatePostRequest {
            name: "new post name".to_string(),
            description: "long long description".to_string(),
            creator_id: 1,
            is_private: false,
            tags: vec!["interesting".to_string(), "tag!".to_string()],
        })
        .await;
    assert!(res.is_ok());
    let posts_table = sqlx::query!("SELECT * FROM posts;")
        .fetch_all(&postsdb.pool)
        .await
        .unwrap();
    assert_eq!(posts_table.len(), 1);
    let posts_row = &posts_table[0];
    assert_eq!(posts_row.name, "new post name");
    assert_eq!(posts_row.description, "long long description");
    assert_eq!(posts_row.creator_id, 1);
    assert_eq!(posts_row.is_private, false);
    assert_eq!(posts_row.tags, vec!["interesting", "tag!"]);
    let res = grpc_client
        .create_post(CreatePostRequest {
            name: "second post".to_string(),
            description: "another long long description".to_string(),
            creator_id: 1,
            is_private: false,
            tags: vec!["interesting".to_string(), "tag!".to_string()],
        })
        .await;
    assert!(res.is_ok());
    let second_post = sqlx::query!("SELECT * FROM posts WHERE name='second post';")
        .fetch_one(&postsdb.pool)
        .await
        .unwrap();
    assert_eq!(second_post.name, "second post");
    assert_eq!(second_post.description, "another long long description");
    assert_eq!(second_post.creator_id, 1);
    assert_eq!(second_post.is_private, false);
    assert_eq!(second_post.tags, vec!["interesting", "tag!"]);
}

#[tokio::test]
async fn posts_get() {
    let postsdb = start_postsdb().await;
    let posts = start_posts(&postsdb.database_url).await;
    let mut grpc_client = PostsServerClient::connect(posts.url).await.unwrap();
    let posts_vec = fill_db(&mut grpc_client).await;
    let creator_id = 1;
    for i in 0..3 {
        let res = grpc_client
            .get_post_by_id(GetByIdRequest {
                post_id: Some(posts_vec[i]),
                creator_id,
            })
            .await
            .unwrap()
            .into_inner();
        assert_eq!(res.name, "name");
        assert_eq!(res.description, "description");
        assert_eq!(res.tags, vec!["tags"]);
    }
    let res = grpc_client
        .get_post_by_id(GetByIdRequest {
            post_id: Some(posts_vec[3]),
            creator_id,
        })
        .await
        .unwrap_err();
    assert_eq!(res.code(), tonic::Code::NotFound);
    let res = grpc_client
        .get_posts_page(GetPostsPageRequest {
            creator_id,
            from: 1,
            limit: 10,
        })
        .await
        .unwrap()
        .into_inner();
    assert_eq!(res.posts.len(), 3);
    let res = grpc_client
        .get_posts_page(GetPostsPageRequest {
            creator_id,
            from: 2,
            limit: 10,
        })
        .await
        .unwrap()
        .into_inner();
    assert_eq!(res.posts.len(), 2);
    let res = grpc_client
        .get_posts_page(GetPostsPageRequest {
            creator_id,
            from: 2,
            limit: 1,
        })
        .await
        .unwrap()
        .into_inner();
    assert_eq!(res.posts.len(), 1);
    let res = grpc_client
        .get_posts_page(GetPostsPageRequest {
            creator_id,
            from: 4,
            limit: 1,
        })
        .await
        .unwrap()
        .into_inner();
    assert!(res.posts.is_empty());
}

#[tokio::test]
async fn posts_update() {
    let postsdb = start_postsdb().await;
    let posts = start_posts(&postsdb.database_url).await;
    let mut grpc_client = PostsServerClient::connect(posts.url).await.unwrap();
    let posts_vec = fill_db(&mut grpc_client).await;
    let creator_id = 1;
    let res = grpc_client
        .update_post(UpdatePostRequest {
            post_id: Some(posts_vec[0]),
            creator_id,
            name: Some("other cool name".to_string()),
            description: None,
            is_private: None,
            tags: vec![],
            field_mask: Some(FieldMask {
                paths: vec!["name".to_string()],
            }),
        })
        .await;
    assert!(res.is_ok());
    let new_post = sqlx::query!("SELECT name, description, is_private, tags FROM posts WHERE id=1;").fetch_one(&postsdb.pool).await.unwrap();
    assert_eq!(new_post.name, "other cool name");
    assert_eq!(new_post.description, "description");
    assert_eq!(new_post.is_private, false);
    assert_eq!(new_post.tags, vec!["tags"]);
    let res = grpc_client
        .update_post(UpdatePostRequest {
            post_id: Some(posts_vec[1]),
            creator_id,
            name: Some("other cool name".to_string()),
            description: Some("other description".to_string()),
            is_private: Some(false),
            tags: vec![],
            field_mask: Some(FieldMask {
                paths: vec!["description".to_string(), "is_private".to_string()],
            }),
        })
        .await;
    assert!(res.is_ok());
    let new_post = sqlx::query!("SELECT name, description, is_private, tags FROM posts WHERE id=2;").fetch_one(&postsdb.pool).await.unwrap();
    assert_eq!(new_post.name, "name");
    assert_eq!(new_post.description, "other description");
    assert_eq!(new_post.is_private, false);
    assert_eq!(new_post.tags, vec!["tags"]);
    let res = grpc_client
        .update_post(UpdatePostRequest {
            post_id: Some(posts_vec[2]),
            creator_id,
            name: Some("other cool name".to_string()),
            description: Some("other description".to_string()),
            is_private: Some(false),
            tags: vec![],
            field_mask: Some(FieldMask {
                paths: vec!["description".to_string(), "is_private".to_string()],
            }),
        })
        .await
        .unwrap_err();
    assert_eq!(res.code(), tonic::Code::PermissionDenied);
    let res = grpc_client
        .update_post(UpdatePostRequest {
            post_id: Some(PostId{post_id: 100}),
            creator_id,
            name: Some("other cool name".to_string()),
            description: Some("other description".to_string()),
            is_private: Some(false),
            tags: vec![],
            field_mask: Some(FieldMask {
                paths: vec!["description".to_string(), "is_private".to_string()],
            }),
        })
        .await
        .unwrap_err();
    assert_eq!(res.code(), tonic::Code::NotFound);
}
