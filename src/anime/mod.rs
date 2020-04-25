use bytes::buf::BufExt as _;
use hyper::{Body, Client};
use hyper::client::HttpConnector;

use crate::{characters, forum, more_info, news, pictures, recommendations, reviews, stats, user_updates};
use crate::anime::episodes::Episode;
use crate::anime::videos::Videos;
use crate::base::{MALItem, RelatedContent, TypeSource};
use crate::characters::Character;
use crate::client::BASE_URL;
use crate::forum::Topic;
use crate::news::News;
use crate::pictures::Picture;
use crate::recommendations::Recommendation;
use crate::reviews::{AnimeReviewer, Review, Reviews};
use crate::stats::{AnimeStats, Stats};
use crate::user_updates::{AnimeUserUpdate, UserUpdates};

pub mod episodes;
pub mod videos;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_anime(mal_id: u32, http_clt: &Client<HttpConnector, Body>) -> Result<Anime> {
    let url = format!("{}/anime/{}", BASE_URL, mal_id).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let mut anime: Anime = serde_json::from_reader(body.reader())?;

    anime.client = http_clt.clone();

    Ok(anime)
}

#[derive(Deserialize, Debug)]
pub struct Anime {
    #[serde(skip)]
    client: Client<HttpConnector, Body>,
    pub request_hash: String,
    pub request_cached: bool,
    pub request_cache_expiry: u32,
    pub mal_id: u32,
    pub url: String,
    pub image_url: Option<String>,
    pub trailer_url: Option<String>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Vec<String>,
    #[serde(rename = "type")]
    pub anime_type: String,
    pub source: String,
    pub episodes: Option<u16>,
    pub status: String,
    pub airing: bool,
    pub aired: Aired,
    pub duration: Option<String>,
    pub rating: Option<String>,
    pub score: Option<f32>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub members: Option<u32>,
    pub favorites: Option<u32>,
    pub synopsis: String,
    pub background: Option<String>,
    pub premiered: Option<String>,
    pub broadcast: Option<String>,
    pub related: RelatedContent,
    pub producers: Vec<MALItem>,
    pub licensors: Vec<MALItem>,
    pub studios: Vec<MALItem>,
    pub genres: Vec<MALItem>,
    pub opening_themes: Vec<String>,
    pub ending_themes: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Aired {
    pub from: Option<String>,
    pub to: Option<String>,
}

impl Anime {
    pub async fn get_characters(&self) -> Result<Vec<Character>> {
        characters::find_characters(TypeSource::Anime(self.mal_id), &self.client).await
    }

    pub async fn get_episodes(&self) -> Result<Vec<Episode>> {
        episodes::find_anime_episodes(&self.mal_id, &self.client).await
    }

    pub async fn get_news(&self) -> Result<Vec<News>> {
        news::find_news(TypeSource::Anime(self.mal_id), &self.client).await
    }

    pub async fn get_pictures(&self) -> Result<Vec<Picture>> {
        pictures::find_pictures(TypeSource::Anime(self.mal_id), &self.client).await
    }

    pub async fn get_videos(&self) -> Result<Videos> {
        videos::find_videos(&self.mal_id, &self.client).await
    }

    pub async fn get_stats(&self) -> Result<AnimeStats> {
        let stats = stats::find_stats(TypeSource::Anime(self.mal_id), &self.client).await?;
        match stats {
            Stats::Anime(stats) => Ok(stats),
            Stats::Manga(_) => Err(Box::from("Expected Anime Stats, but returned Manga Stats")),
        }
    }

    pub async fn get_forum(&self) -> Result<Vec<Topic>> {
        forum::find_forum(TypeSource::Anime(self.mal_id), &self.client).await
    }

    pub async fn get_more_info(&self) -> Result<Option<String>> {
        more_info::find_more_info(TypeSource::Anime(self.mal_id), &self.client).await
    }

    pub async fn get_reviews(&self, page: &u16) -> Result<Vec<Review<AnimeReviewer>>> {
        let reviews = reviews::find_reviews(TypeSource::Anime(self.mal_id), page, &self.client).await?;
        match reviews {
            Reviews::Anime(reviews) => Ok(reviews),
            Reviews::Manga(_) => Err(Box::from("Expected Anime Reviews, but returned Manga Reviews")),
        }
    }

    pub async fn get_recommendations(&self) -> Result<Vec<Recommendation>> {
        recommendations::find_recommendations(TypeSource::Anime(self.mal_id), &self.client).await
    }

    pub async fn get_user_updates(&self, page: &u16) -> Result<Vec<AnimeUserUpdate>> {
        let user_updates = user_updates::find_user_updates(TypeSource::Anime(self.mal_id), page, &self.client).await?;
        match user_updates {
            UserUpdates::Anime(user_updates) => Ok(user_updates),
            UserUpdates::Manga(_) => Err(Box::from("Expected Anime User Updates, but returned Manga User Updates")),
        }
    }
}