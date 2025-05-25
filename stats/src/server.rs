use crate::{Error, db};
use clickhouse::Client;
use stats_server::stats_server::Stats;
use stats_server::*;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub mod stats_server {
    tonic::include_proto!("stats");
}

pub struct MyStatsServer {
    pub client: Client,
}

#[tonic::async_trait]
impl Stats for MyStatsServer {
    type DynamicsStream = ReceiverStream<std::result::Result<DynamicsResponse, Status>>;
    type GetTop10PostsStream = ReceiverStream<std::result::Result<GetTop10PostsResponse, Status>>;
    type GetTop10UsersStream = ReceiverStream<std::result::Result<GetTop10UsersResponse, Status>>;
    async fn count_activity(
        &self,
        request: Request<CountActivityRequest>,
    ) -> std::result::Result<Response<CountActivityResponse>, Status> {
        let response = db::count_stats(&self.client, request.into_inner().post_id).await?;
        Ok(Response::new(CountActivityResponse{comments: response.comments.unwrap(), likes: response.likes.unwrap(), views: response.views.unwrap()}))
    }
    async fn dynamics(
        &self,
        request: Request<DynamicsRequest>,
    ) -> std::result::Result<Response<Self::DynamicsStream>, Status> {
        let inner = request.into_inner();
        let (tx, rx) = mpsc::channel(4);
        match inner.r#type.try_into() {
            Ok(activity_type) => match activity_type {
                ActivityType::Views => {
                    let mut cursor = db::dynamics_views(&self.client, inner.post_id).await?;
                    tokio::spawn(async move {
                        loop {
                            let nxt = cursor.next().await;
                            match nxt {
                                Ok(ok_row) => {
                                    if let Some(row) = ok_row {
                                        tx.send(Ok(DynamicsResponse {
                                            count: row.count as u64,
                                            day: row.day.to_string(),
                                        }))
                                        .await
                                        .unwrap();
                                    } else {
                                        break;
                                    }
                                }
                                Err(err) => {
                                    tx.send(Err(Into::<Error>::into(err).into())).await.unwrap();
                                    break;
                                }
                            }
                        }
                    });
                }
                ActivityType::Likes => {
                    let mut cursor = db::dynamics_likes(&self.client, inner.post_id).await?;
                    tokio::spawn(async move {
                        loop {
                            let nxt = cursor.next().await;
                            match nxt {
                                Ok(ok_row) => {
                                    if let Some(row) = ok_row {
                                        tx.send(Ok(DynamicsResponse {
                                            count: row.count as u64,
                                            day: row.day.to_string(),
                                        }))
                                        .await
                                        .unwrap();
                                    } else {
                                        break;
                                    }
                                }
                                Err(err) => {
                                    tx.send(Err(Into::<Error>::into(err).into())).await.unwrap();
                                    break;
                                }
                            }
                        }
                    });
                }
                ActivityType::Comments => {
                    let mut cursor = db::dynamics_comments(&self.client, inner.post_id).await?;
                    tokio::spawn(async move {
                        loop {
                            let nxt = cursor.next().await;
                            match nxt {
                                Ok(ok_row) => {
                                    if let Some(row) = ok_row {
                                        tx.send(Ok(DynamicsResponse {
                                            count: row.count as u64,
                                            day: row.day.to_string(),
                                        }))
                                        .await
                                        .unwrap();
                                    } else {
                                        break;
                                    }
                                }
                                Err(err) => {
                                    tx.send(Err(Into::<Error>::into(err).into())).await.unwrap();
                                    break;
                                }
                            }
                        }
                    });
                }
            },
            Err(_) => {
                return Err(Status::invalid_argument("Wrong activity type"));
            }
        }
        Ok(Response::new(Self::DynamicsStream::new(rx)))
    }
    async fn get_top10_posts(
        &self,
        request: Request<GetTop10PostsRequest>,
    ) -> std::result::Result<Response<Self::GetTop10PostsStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        match request.into_inner().r#type.try_into() {
            Ok(activity_type) => match activity_type {
                ActivityType::Views => {
                    let mut cursor = db::get_top_10_posts_views(&self.client).await?;
                    tokio::spawn(async move {
                        loop {
                            let nxt = cursor.next().await;
                            match nxt {
                                Ok(ok_row) => {
                                    if let Some(row) = ok_row {
                                        tx.send(Ok(GetTop10PostsResponse {
                                            post_id: row.post_id,
                                        }))
                                        .await
                                        .unwrap();
                                    } else {
                                        break;
                                    }
                                }
                                Err(err) => {
                                    tx.send(Err(Into::<Error>::into(err).into())).await.unwrap();
                                    break;
                                }
                            }
                        }
                    });
                }
                ActivityType::Likes => {
                    let mut cursor = db::get_top_10_posts_likes(&self.client).await?;
                    tokio::spawn(async move {
                        loop {
                            let nxt = cursor.next().await;
                            match nxt {
                                Ok(ok_row) => {
                                    if let Some(row) = ok_row {
                                        tx.send(Ok(GetTop10PostsResponse {
                                            post_id: row.post_id,
                                        }))
                                        .await
                                        .unwrap();
                                    } else {
                                        break;
                                    }
                                }
                                Err(err) => {
                                    tx.send(Err(Into::<Error>::into(err).into())).await.unwrap();
                                    break;
                                }
                            }
                        }
                    });
                }
                ActivityType::Comments => {
                    let mut cursor = db::get_top_10_posts_comments(&self.client).await?;
                    tokio::spawn(async move {
                        loop {
                            let nxt = cursor.next().await;
                            match nxt {
                                Ok(ok_row) => {
                                    if let Some(row) = ok_row {
                                        tx.send(Ok(GetTop10PostsResponse {
                                            post_id: row.post_id,
                                        }))
                                        .await
                                        .unwrap();
                                    } else {
                                        break;
                                    }
                                }
                                Err(err) => {
                                    tx.send(Err(Into::<Error>::into(err).into())).await.unwrap();
                                    break;
                                }
                            }
                        }
                    });
                }
            },
            Err(_) => {
                return Err(Status::invalid_argument("Wrong activity type"));
            }
        }
        Ok(Response::new(Self::GetTop10PostsStream::new(rx)))
    }
    async fn get_top10_users(
        &self,
        request: Request<GetTop10UsersRequest>,
    ) -> std::result::Result<Response<Self::GetTop10UsersStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        match request.into_inner().r#type.try_into() {
            Ok(activity_type) => match activity_type {
                ActivityType::Views => {
                    let mut cursor = db::get_top_10_users_views(&self.client).await?;
                    tokio::spawn(async move {
                        loop {
                            let nxt = cursor.next().await;
                            match nxt {
                                Ok(ok_row) => {
                                    if let Some(row) = ok_row {
                                        tx.send(Ok(GetTop10UsersResponse {
                                            user_id: row.user_id,
                                        }))
                                        .await
                                        .unwrap();
                                    } else {
                                        break;
                                    }
                                }
                                Err(err) => {
                                    tx.send(Err(Into::<Error>::into(err).into())).await.unwrap();
                                    break;
                                }
                            }
                        }
                    });
                }
                ActivityType::Likes => {
                    let mut cursor = db::get_top_10_users_likes(&self.client).await?;
                    tokio::spawn(async move {
loop {
                            let nxt = cursor.next().await;
                            match nxt {
                                Ok(ok_row) => {
                                    if let Some(row) = ok_row {
                                        tx.send(Ok(GetTop10UsersResponse {
                                            user_id: row.user_id,
                                        }))
                                        .await
                                        .unwrap();
                                    } else {
                                        break;
                                    }
                                }
                                Err(err) => {
                                    tx.send(Err(Into::<Error>::into(err).into())).await.unwrap();
                                    break;
                                }
                            }
                        }
                    });
                }
                ActivityType::Comments => {
                    let mut cursor = db::get_top_10_users_comments(&self.client).await?;
                    tokio::spawn(async move {
loop {
                            let nxt = cursor.next().await;
                            match nxt {
                                Ok(ok_row) => {
                                    if let Some(row) = ok_row {
                                        tx.send(Ok(GetTop10UsersResponse {
                                            user_id: row.user_id,
                                        }))
                                        .await
                                        .unwrap();
                                    } else {
                                        break;
                                    }
                                }
                                Err(err) => {
                                    tx.send(Err(Into::<Error>::into(err).into())).await.unwrap();
                                    break;
                                }
                            }
                        }
                    });
                }
            },
            Err(_) => {
                return Err(Status::invalid_argument("Wrong activity type"));
            }
        }
        Ok(Response::new(Self::GetTop10UsersStream::new(rx)))
    }
}
