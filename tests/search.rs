extern crate jikan_rs;
#[macro_use]
extern crate lazy_static;

use jikan_rs::client::Jikan;
use jikan_rs::prelude::*;
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref JIKAN: Jikan = Jikan::new();
}

#[tokio::test]
async fn should_search_for_anime_one_piece() {
    thread::sleep(Duration::from_secs(3));
    let query = SearchQueryBuilder::new(SearchSource::Anime)
        .name("one p")
        .type_source(SearchSourceType::Anime(AnimeType::TV))
        .genre(Genres::Anime(vec![AnimeGenre::Shounen, AnimeGenre::Adventure]));

    let anime = JIKAN.search(query)
        .await.unwrap();

    let anime = match anime {
        SearchResultEnum::Anime(result) => Some(result),
        _ => None
    }.unwrap();

    assert_eq!(anime.results[0].title, "One Piece");
}

#[tokio::test]
async fn should_search_for_manga_berserk() {
    thread::sleep(Duration::from_secs(3));
    let query = SearchQueryBuilder::new(SearchSource::Manga)
        .name("berser")
        .type_source(SearchSourceType::Manga(MangaType::Manga));

    let manga = JIKAN.search(query)
        .await.unwrap();

    let manga = match manga {
        SearchResultEnum::Manga(result) => Some(result),
        _ => None
    }.unwrap();

    assert_eq!(manga.results[0].title, "Berserk");
}

#[tokio::test]
async fn should_search_for_person_masashi_kishimoto() {
    thread::sleep(Duration::from_secs(3));
    let query = SearchQueryBuilder::new(SearchSource::Person)
        .name("岸本 斉史");

    let person = JIKAN.search(query)
        .await.unwrap();

    let person = match person {
        SearchResultEnum::Person(result) => Some(result),
        _ => None
    }.unwrap();

    assert_eq!(person.results[0].name, "Masashi Kishimoto");
}

#[tokio::test]
async fn should_search_for_character_lelouch_lamperouge() {
    thread::sleep(Duration::from_secs(3));
    let query = SearchQueryBuilder::new(SearchSource::Character)
        .name("ルルーシュ");

    let characters = JIKAN.search(query)
        .await.unwrap();

    let characters = match characters {
        SearchResultEnum::Character(result) => Some(result),
        _ => None
    }.unwrap();

    assert_eq!(characters.results[0].name, "Lamperouge, Lelouch");
}