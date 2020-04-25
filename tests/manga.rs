extern crate jikan_rs;
#[macro_use]
extern crate lazy_static;

use jikan_rs::client::JikanClient;

lazy_static! {
    static ref JIKAN_CL: JikanClient = JikanClient::new();
}

#[tokio::test]
async fn should_get_manga_characters() {
    let characters = JIKAN_CL.find_manga(1)
        .await.unwrap()
        .get_characters()
        .await.unwrap();
    assert!(!characters.is_empty());
}

#[tokio::test]
async fn should_get_manga_news() {
    let news = JIKAN_CL.find_manga(1)
        .await.unwrap()
        .get_news()
        .await.unwrap();
    assert!(!news.is_empty());
}

#[tokio::test]
async fn should_get_manga_pictures() {
    let pictures = JIKAN_CL.find_manga(1)
        .await.unwrap()
        .get_pictures()
        .await.unwrap();
    assert!(!pictures.is_empty());
}

#[tokio::test]
async fn should_get_manga_stats() {
    let stats = JIKAN_CL.find_manga(1)
        .await.unwrap()
        .get_stats()
        .await.unwrap();
    assert!(stats.total > 0);
}

#[tokio::test]
async fn should_get_manga_forum() {
    let topics = JIKAN_CL.find_manga(1)
        .await.unwrap()
        .get_forum()
        .await.unwrap();
    assert!(!topics.is_empty());
}

#[tokio::test]
async fn should_get_more_manga_info() {
    let more_info = JIKAN_CL.find_manga(2)
        .await.unwrap()
        .get_more_info()
        .await.unwrap().unwrap();
    assert!(!more_info.is_empty());
}

#[tokio::test]
async fn should_get_manga_reviews() {
    let reviews = JIKAN_CL.find_manga(1)
        .await.unwrap()
        .get_reviews(&1)
        .await.unwrap();
    assert!(reviews.len() > 0 && reviews.len() <= 20);
}

#[tokio::test]
async fn should_get_manga_recommendations() {
    let recommendations = JIKAN_CL.find_manga(1)
        .await.unwrap()
        .get_recommendations()
        .await.unwrap();
    assert!(!recommendations.is_empty());
}

#[tokio::test]
async fn should_get_manga_user_updates() {
    let user_updates = JIKAN_CL.find_manga(1)
        .await.unwrap()
        .get_user_updates(&1)
        .await.unwrap();
    assert!(!user_updates.is_empty());
}