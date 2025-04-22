use crate::{Result, Error};
use sqlx::{PgPool, QueryBuilder, types::chrono::NaiveDateTime};

#[derive(Debug)]
pub struct Post {
    pub id: i64,
    pub name: String,
    pub creator_id: i32,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_private: bool,
    pub tags: Vec<String>
}

pub async fn insert_post(
    pool: &PgPool,
    name: &str,
    creator_id: i32,
    description: &str,
    is_private: bool,
    tags: &[String]
) -> Result<i64> {
    Ok(sqlx::query!("INSERT INTO posts (name, creator_id, description, is_private, tags) VALUES ($1, $2, $3, $4, $5) RETURNING id;", name, creator_id, description, is_private, tags).fetch_one(pool).await?.id)
}

pub async fn update_post(
    pool: &PgPool,
    id: i64,
    creator_id: i32,
    name: Option<&str>,
    description: Option<&str>,
    is_private: Option<bool>,
    tags: Option<&[String]>
) -> Result<()> {
    let real_creator_id = sqlx::query!("SELECT creator_id FROM posts WHERE id=$1;", id).fetch_optional(pool).await?;
    match real_creator_id {
        Some(id) => {
            if id.creator_id != creator_id {
                return Err(Error::Forbidden("Forbidden".to_owned()));
            }
        }
        None => {
            return Err(Error::NotFound("Post with given id is not found".to_owned()));
        }
    }
    let mut query_builder = QueryBuilder::new("UPDATE posts SET");
    let mut is_first = true;
    if name.is_some() {
        if is_first {
            is_first = false;
        } else {
            query_builder.push(",");
        }
        query_builder.push(" name=");
        query_builder.push_bind(name);
    }
    if description.is_some() {
        if is_first {
            is_first = false;
        } else {
            query_builder.push(",");
        }
        query_builder.push(" description=");
        query_builder.push_bind(description);
    }
    if is_private.is_some() {
        if is_first {
            is_first = false;
        } else {
            query_builder.push(",");
        }
        query_builder.push(" is_private=");
        query_builder.push_bind(is_private);
    }
    if tags.is_some() {
        if is_first {
            is_first = false;
        } else {
            query_builder.push(",");
        }
        query_builder.push(" tags=");
        query_builder.push_bind(tags);
    }
    if !is_first {
        query_builder.push(",");
    }
    query_builder.push(" updated_at=now()");
    query_builder.push(" WHERE id=");
    query_builder.push_bind(id);
    query_builder.push(";");
    query_builder.build().execute(pool).await?;
    Ok(())
}

pub async fn delete_post(pool: &PgPool, id: i64, creator_id: i32) -> Result<()> {
    let real_creator_id = sqlx::query!("SELECT creator_id FROM posts WHERE id=$1;", id).fetch_optional(pool).await?;
    match real_creator_id {
        Some(id) => {
            if id.creator_id != creator_id {
                return Err(Error::Forbidden("Forbidden".to_owned()));
            }
        }
        None => {
            return Err(Error::NotFound("Post with given id is not found".to_owned()));
        }
    }

    sqlx::query!("DELETE FROM posts WHERE id=$1;", id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_post(pool: &PgPool, id: i64, creator_id: i32) -> Result<Post> {
    Ok(sqlx::query_as!(Post, "SELECT id, name, creator_id, description, created_at, updated_at, is_private, tags FROM posts WHERE id=$1 AND (creator_id=$2 OR NOT is_private);", id, creator_id).fetch_one(pool).await?)
}

pub async fn get_page(pool: &PgPool, creator_id: i32, from: i64, limit: i64) -> Result<Vec<Post>> {
    Ok(sqlx::query_as!(Post, "SELECT id, name, creator_id, description, created_at, updated_at, is_private, tags FROM posts WHERE id>=$1 AND (creator_id=$3 OR NOT is_private) ORDER BY id LIMIT $2;", from, limit, creator_id).fetch_all(pool).await?)
}
