mod containers;
mod model;

use std::time::Duration;

use containers::start_gateway;
use model::*;
use reqwest::Client;

#[tokio::test]
async fn first_scenario() {
    let gateway = start_gateway().await;
    let client = Client::builder().cookie_store(true).build().unwrap();
    let user = User {
        username: "anton".to_string(),
        password: "aboba".to_string(),
        name: "anton".to_string(),
        surname: "ll".to_string(),
        birthdate: "2025-06-04".to_string(),
        status: "I'm good".to_string(),
        mail: "boba@ya.ru".to_string(),
        phone: "88005553535".to_string(),
    };
    let res = client
        .post(gateway.get_addr("user"))
        .json(&user)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let res = client
        .get(gateway.get_addr("login"))
        .query(&[("username", &user.username), ("password", &user.password)])
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.cookies().count(), 1);
    let post = Post {
        name: "Cool post name!".to_string(),
        description: "This post is about a very cool thing!".to_string(),
        is_private: false,
        tags: vec!["cool".to_string(), "thing".to_string()],
    };
    let res = client.post(gateway.get_addr("post"))
        .json(&post)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<PostId>().await.unwrap().post_id, 1);
    let post2 = Post {
        name: "Not cool post name!".to_string(),
        description: "This post is about a very cool thing!".to_string(),
        is_private: false,
        tags: vec!["cool".to_string(), "thing".to_string()],
    };
    let res = client.post(gateway.get_addr("post"))
        .json(&post2)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<PostId>().await.unwrap().post_id, 2);
    let post_id = PostId{post_id: 1};
    let res = client.post(gateway.get_addr("like")).json(&post_id)
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    let post_id = PostId{post_id: 1};
    let res = client.post(gateway.get_addr("like")).json(&post_id)
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    let post_id = PostId{post_id: 2};
    let res = client.post(gateway.get_addr("like")).json(&post_id)
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    tokio::time::sleep(Duration::from_secs(10)).await;
    let res = client.get(gateway.get_addr("stats/1")).send().await.unwrap();
    assert_eq!(res.status(), 200);
    let stats = res.json::<Acitivity>().await.unwrap();
    assert_eq!(stats.comments, 0);
    assert_eq!(stats.likes, 2);
    assert_eq!(stats.views, 0);
}

#[tokio::test]
async fn second_scenario() {
    let gateway = start_gateway().await;
    let client = Client::builder().cookie_store(true).build().unwrap();
    let client2 = Client::builder().cookie_store(true).build().unwrap();
    let user = User {
        username: "anton".to_string(),
        password: "aboba".to_string(),
        name: "anton".to_string(),
        surname: "ll".to_string(),
        birthdate: "2025-06-04".to_string(),
        status: "I'm good".to_string(),
        mail: "boba@ya.ru".to_string(),
        phone: "88005553535".to_string(),
    };
    let user2 = User {
        username: "bob".to_string(),
        password: "aboba".to_string(),
        name: "anton".to_string(),
        surname: "ll".to_string(),
        birthdate: "2025-06-04".to_string(),
        status: "I'm good".to_string(),
        mail: "boba@ya.ru".to_string(),
        phone: "88005553535".to_string(),
    };
    let res = client
        .post(gateway.get_addr("user"))
        .json(&user)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let res = client
        .get(gateway.get_addr("login"))
        .query(&[("username", &user.username), ("password", &user.password)])
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.cookies().count(), 1);
    let res = client
        .post(gateway.get_addr("user"))
        .json(&user2)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let res = client2
        .get(gateway.get_addr("login"))
        .query(&[("username", &user2.username), ("password", &user2.password)])
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.cookies().count(), 1);
    let post = Post {
        name: "Cool post name!".to_string(),
        description: "This post is about a very cool thing!".to_string(),
        is_private: false,
        tags: vec!["cool".to_string(), "thing".to_string()],
    };
    let res = client.post(gateway.get_addr("post"))
        .json(&post)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<PostId>().await.unwrap().post_id, 1);
    let post2 = Post {
        name: "Not cool post name!".to_string(),
        description: "This post is about a very cool thing!".to_string(),
        is_private: true,
        tags: vec!["cool".to_string(), "thing".to_string()],
    };
    let res = client.post(gateway.get_addr("post"))
        .json(&post2)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<PostId>().await.unwrap().post_id, 2);
    let post_id = PostId{post_id: 1};
    let res = client.post(gateway.get_addr("like")).json(&post_id)
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    let post_id = PostId{post_id: 1};
    let res = client.post(gateway.get_addr("like")).json(&post_id)
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    let post_id = PostId{post_id: 2};
    let res = client.post(gateway.get_addr("like")).json(&post_id)
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    tokio::time::sleep(Duration::from_secs(10)).await;
    let res = client2.get(gateway.get_addr("stats/1")).send().await.unwrap();
    assert_eq!(res.status(), 200);
    let stats = res.json::<Acitivity>().await.unwrap();
    assert_eq!(stats.comments, 0);
    assert_eq!(stats.likes, 2);
    assert_eq!(stats.views, 0);
    let res = client2.get(gateway.get_addr("post/2")).send().await.unwrap();
    assert_eq!(res.status(), 404);
}

#[tokio::test]
async fn third_scenario() {
    let gateway = start_gateway().await;
    let client = Client::builder().cookie_store(true).build().unwrap();
    let client2 = Client::builder().cookie_store(true).build().unwrap();
    let user = User {
        username: "anton".to_string(),
        password: "aboba".to_string(),
        name: "anton".to_string(),
        surname: "ll".to_string(),
        birthdate: "2025-06-04".to_string(),
        status: "I'm good".to_string(),
        mail: "boba@ya.ru".to_string(),
        phone: "88005553535".to_string(),
    };
    let user2 = User {
        username: "bob".to_string(),
        password: "aboba".to_string(),
        name: "anton".to_string(),
        surname: "ll".to_string(),
        birthdate: "2025-06-04".to_string(),
        status: "I'm good".to_string(),
        mail: "boba@ya.ru".to_string(),
        phone: "88005553535".to_string(),
    };
    let res = client
        .post(gateway.get_addr("user"))
        .json(&user)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let res = client
        .get(gateway.get_addr("login"))
        .query(&[("username", &user.username), ("password", &user.password)])
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.cookies().count(), 1);
    let res = client2
        .post(gateway.get_addr("user"))
        .json(&user2)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let res = client2
        .get(gateway.get_addr("login"))
        .query(&[("username", &user2.username), ("password", &user2.password)])
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.cookies().count(), 1);
    let post = Post {
        name: "Cool post name!".to_string(),
        description: "This post is about a very cool thing!".to_string(),
        is_private: false,
        tags: vec!["cool".to_string(), "thing".to_string()],
    };
    let res = client.post(gateway.get_addr("post"))
        .json(&post)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<PostId>().await.unwrap().post_id, 1);
    let post2 = Post {
        name: "Not cool post name!".to_string(),
        description: "This post is about a very cool thing!".to_string(),
        is_private: true,
        tags: vec!["cool".to_string(), "thing".to_string()],
    };
    let res = client.post(gateway.get_addr("post"))
        .json(&post2)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<PostId>().await.unwrap().post_id, 2);
    let post_id = PostId{post_id: 1};
    let res = client.post(gateway.get_addr("like")).json(&post_id)
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    let post_id = PostId{post_id: 1};
    let res = client2.post(gateway.get_addr("like")).json(&post_id)
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    let post_id = PostId{post_id: 2};
    let res = client.post(gateway.get_addr("like")).json(&post_id)
        .send().await.unwrap();
    assert_eq!(res.status(), 200);
    tokio::time::sleep(Duration::from_secs(10)).await;
    let res = client2.get(gateway.get_addr("stats/1")).send().await.unwrap();
    assert_eq!(res.status(), 200);
    let stats = res.json::<Acitivity>().await.unwrap();
    assert_eq!(stats.comments, 0);
    assert_eq!(stats.likes, 2);
    assert_eq!(stats.views, 0);
    let res = client2.get(gateway.get_addr("post/2")).send().await.unwrap();
    assert_eq!(res.status(), 404);
    let change_privacy = ChangePrivacy{is_private: false};
    let res = client2.put(gateway.get_addr("post/2")).json(&change_privacy).send().await.unwrap();
    assert_eq!(res.status(), 403);
    let res = client.put(gateway.get_addr("post/2")).json(&change_privacy).send().await.unwrap();
    assert_eq!(res.status(), 200);
    let res = client2.get(gateway.get_addr("post/2")).send().await.unwrap();
    assert_eq!(res.status(), 200);
    let res = client2.get(gateway.get_addr("top/posts/Likes")).send().await.unwrap();
    assert_eq!(res.status(), 200);
    let inner = res.json::<Vec<PostId>>().await.unwrap();
    assert_eq!(inner.len(), 2);
    assert_eq!(inner[0].post_id, 1);
    assert_eq!(inner[1].post_id, 2);
}
