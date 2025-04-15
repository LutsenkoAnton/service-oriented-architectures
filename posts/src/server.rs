use crate::db;
use posts_server::posts_server_server::PostsServer;
use posts_server::*;
use prost_types::Timestamp;
use sqlx::PgPool;
use tonic::{Request, Response, Status};

pub mod posts_server {
    tonic::include_proto!("posts");
}

#[derive(Debug)]
pub struct MyPostsServer {
    pub pool: PgPool,
}

#[tonic::async_trait]
impl PostsServer for MyPostsServer {
    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> std::result::Result<Response<PostId>, Status> {
        let inner = request.into_inner();
        let id = db::insert_post(
            &self.pool,
            &inner.name,
            inner.creator_id,
            &inner.description,
            inner.is_private,
            &inner.tags,
        )
        .await?;
        Ok(Response::new(PostId { post_id: id }))
    }
    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        let inner = request.into_inner();
        db::delete_post(&self.pool, inner.post_id.unwrap().post_id, inner.creator_id).await?;
        Ok(Response::new(()))
    }
    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> std::result::Result<Response<()>, Status> {
        let mut name: Option<&str> = None;
        let mut description: Option<&str> = None;
        let mut is_private: Option<bool> = None;
        let mut tags: Option<&[String]> = None;
        let inner = request.into_inner();
        for field_mask in inner.field_mask.unwrap().paths {
            for field in field_mask.split(",") {
                if field == "name" {
                    name = Some(inner.name.as_ref().unwrap());
                } else if field == "description" {
                    description = Some(inner.description.as_ref().unwrap());
                } else if field == "is_private" {
                    is_private = Some(inner.is_private.unwrap());
                } else if field == "tags" {
                    tags = Some(&inner.tags);
                }
            }
        }
        db::update_post(&self.pool, inner.post_id.unwrap().post_id, inner.creator_id, name, description, is_private, tags).await?;
        Ok(Response::new(()))
    }
    async fn get_post_by_id(
        &self,
        request: Request<GetByIdRequest>,
    ) -> std::result::Result<Response<Post>, Status> {
        let inner = request.into_inner();
        let db_post = db::get_post(&self.pool, inner.post_id.unwrap().post_id, inner.creator_id).await?;
        Ok(Response::new(Post {
            id: Some(PostId {
                post_id: db_post.id,
            }),
            name: db_post.name,
            description: db_post.description,
            creator_id: db_post.creator_id,
            creation_time: Some(Timestamp {
        seconds: db_post.created_at.and_utc().timestamp(),
        nanos: db_post.created_at.and_utc().timestamp_subsec_nanos() as i32,
    }),
            update_time: Some(Timestamp {
        seconds: db_post.updated_at.and_utc().timestamp(),
        nanos: db_post.updated_at.and_utc().timestamp_subsec_nanos() as i32,
    }),
            is_private: db_post.is_private,
            tags: db_post.tags,
        }))
    }

    async fn get_posts_page(
        &self,
        request: Request<GetPostsPageRequest>,
    ) -> std::result::Result<Response<Posts>, Status> {
        let inner = request.into_inner();
        let page = db::get_page(&self.pool, inner.creator_id, inner.from.into(), inner.limit.into()).await?;
        let posts = page.iter().map(|db_post| {
            Post {
            id: Some(PostId {
                post_id: db_post.id,
            }),
            name: db_post.name.clone(),
            description: db_post.description.clone(),
            creator_id: db_post.creator_id,
            creation_time: Some(Timestamp {
        seconds: db_post.created_at.and_utc().timestamp(),
        nanos: db_post.created_at.and_utc().timestamp_subsec_nanos() as i32,
    }),
            update_time: Some(Timestamp {
        seconds: db_post.updated_at.and_utc().timestamp(),
        nanos: db_post.updated_at.and_utc().timestamp_subsec_nanos() as i32,
    }),
            is_private: db_post.is_private,
            tags: db_post.tags.clone(),
        }
        }).collect();
        Ok(Response::new(Posts{posts}))

    }
}
