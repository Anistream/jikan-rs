use bytes::buf::BufExt;
use hyper::{Body, Client};
use hyper::client::HttpConnector;

use crate::{news, pictures, stats, forum, more_info, reviews, recommendations, user_updates};
use crate::base::{MALTypeItem, RelatedContent, SourceType, MALRoleItem};
use crate::client::BASE_URL;
use crate::news::News;
use crate::pictures::Picture;
use crate::stats::{Stats, MangaStats};
use crate::forum::Topic;
use crate::reviews::{Review, MangaReviewer, Reviews};
use crate::recommendations::Recommendation;
use crate::user_updates::{MangaUserUpdate, UserUpdates};

pub mod characters;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_manga(mal_id: u32, http_clt: &Client<HttpConnector, Body>) -> Result<Manga> {
    let url = format!("{}/manga/{}", BASE_URL, mal_id).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let mut manga: Manga = serde_json::from_reader(body.reader())?;

    manga.client = http_clt.clone();

    Ok(manga)
}

#[derive(Deserialize, Debug)]
pub struct Manga {
    #[serde(skip)]
    client: Client<HttpConnector, Body>,
    pub request_hash: String,
    pub request_cached: bool,
    pub request_cache_expiry: u32,
    pub mal_id: u32,
    pub url: String,
    pub image_url: Option<String>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Vec<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub manga_type: String,
    pub volumes: Option<u32>,
    pub chapters: Option<u32>,
    pub publishing: bool,
    pub published: Published,
    pub rank: Option<u32>,
    pub score: Option<f32>,
    pub scored_by: Option<u32>,
    pub popularity: Option<u32>,
    pub members: Option<u32>,
    pub favorites: Option<u32>,
    pub synopsis: String,
    pub background: Option<String>,
    pub related: RelatedContent,
    pub genres: Vec<MALTypeItem>,
    pub authors: Vec<MALTypeItem>,
    pub serializations: Vec<MALTypeItem>,
}

#[derive(Deserialize, Debug)]
pub struct Published {
    pub from: Option<String>,
    pub to: Option<String>,
}

impl Manga {
    pub async fn get_characters(&self) -> Result<Vec<MALRoleItem>> {
        characters::find_characters(&self.mal_id, &self.client).await
    }

    pub async fn get_news(&self) -> Result<Vec<News>> {
        news::find_news(SourceType::Manga(self.mal_id), &self.client).await
    }

    pub async fn get_pictures(&self) -> Result<Vec<Picture>> {
        pictures::find_pictures(SourceType::Manga(self.mal_id), &self.client).await
    }

    pub async fn get_stats(&self) -> Result<MangaStats> {
        let stats = stats::find_stats(SourceType::Manga(self.mal_id), &self.client).await?;
        match stats {
            Stats::Manga(stats) => Ok(stats),
            Stats::Anime(_) => Err(Box::from("Expected Manga Stats, but returned Anime Stats")),
        }
    }

    pub async fn get_forum(&self) -> Result<Vec<Topic>> {
        forum::find_forum(SourceType::Manga(self.mal_id), &self.client).await
    }

    pub async fn get_more_info(&self) -> Result<Option<String>> {
        more_info::find_more_info(SourceType::Manga(self.mal_id), &self.client).await
    }

    pub async fn get_reviews(&self, page: &u16) -> Result<Vec<Review<MangaReviewer>>> {
        let reviews = reviews::find_reviews(SourceType::Manga(self.mal_id), page, &self.client).await?;
        match reviews {
            Reviews::Manga(reviews) => Ok(reviews),
            Reviews::Anime(_) => Err(Box::from("Expected Manga Reviews, but returned Anime Reviews")),
        }
    }

    pub async fn get_recommendations(&self) -> Result<Vec<Recommendation>> {
        recommendations::find_recommendations(SourceType::Manga(self.mal_id), &self.client).await
    }

    pub async fn get_user_updates(&self, page: &u16) -> Result<Vec<MangaUserUpdate>> {
        let user_updates = user_updates::find_user_updates(SourceType::Manga(self.mal_id), page, &self.client).await?;
        match user_updates {
            UserUpdates::Manga(user_updates) => Ok(user_updates),
            UserUpdates::Anime(_) => Err(Box::from("Expected Manga User Updates, but returned Anime User Updates")),
        }
    }
}