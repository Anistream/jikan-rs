use bytes::buf::BufExt;
use hyper::{Body, Client};
use hyper::client::HttpConnector;
use crate::base::MALTypeItem;
use crate::client::BASE_URL;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_season(season: Season, http_clt: &Client<HttpConnector, Body>) -> Result<SeasonResult> {
    let url = format!("{}/season/{}", BASE_URL, season.get_uri()).parse()?;
    let res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let season_result: SeasonResult = serde_json::from_reader(body.reader())?;

    Ok(season_result)
}

pub enum Season {
    Winter(u16),
    Fall(u16),
    Summer(u16),
    Spring(u16),
    Later
}

impl Season {
    pub fn get_uri(&self) -> String {
        match self {
            Season::Winter(year) => format!("{}/winter", year),
            Season::Fall(year) => format!("{}/fall", year),
            Season::Summer(year) => format!("{}/summer", year),
            Season::Spring(year) => format!("{}/spring", year),
            Season::Later => format!("later"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SeasonResult {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    pub season_name: String,
    pub season_year: Option<u16>,
    #[serde(rename = "anime")]
    pub animes: Vec<AnimeSeason>,
}

#[derive(Deserialize, Debug)]
pub struct AnimeSeason {
    pub mal_id: u32,
    pub url: String,
    pub title: String,
    pub image_url: Option<String>,
    pub synopsis: String,
    #[serde(rename = "type")]
    pub anime_type: String,
    pub airing_start: Option<String>,
    pub episodes: Option<u16>,
    pub members: Option<u32>,
    pub genres: Vec<MALTypeItem>,
    pub source: String,
    pub producers: Vec<MALTypeItem>,
    pub score: Option<f32>,
    pub licensors: Vec<String>,
    pub r18: bool,
    pub kids: bool,
    pub continuing: bool,
}