use std::fmt;

use bytes::buf::BufExt;
use hyper::{Body, Client};
use hyper::client::HttpConnector;

use crate::base::AnimeInfo;
use crate::client::BASE_URL;
use hyper::{body::HttpBody as _};
use tokio::io::{self, AsyncWriteExt as _};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn find_schedule(schedule_on: ScheduleOn, http_clt: &Client<HttpConnector, Body>) -> Result<Schedule> {
    let url = format!("{}/schedule/{}", BASE_URL, schedule_on.get_uri()).parse()?;
    let mut res = http_clt.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let schedule: Schedule = serde_json::from_reader(body.reader())?;

    Ok(schedule)
}

#[derive(Debug)]
pub enum ScheduleOn {
    All,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    Other,
    Unknown,
}

impl fmt::Display for ScheduleOn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ScheduleOn {
    fn get_uri(&self) -> String {
        match self {
            ScheduleOn::All => String::new(),
            _ => self.to_string().to_lowercase(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Schedule {
    request_hash: String,
    request_cached: bool,
    request_cache_expiry: u32,
    #[serde(default = "default_content")]
    pub monday: Vec<AnimeInfo>,
    #[serde(default = "default_content")]
    pub tuesday: Vec<AnimeInfo>,
    #[serde(default = "default_content")]
    pub wednesday: Vec<AnimeInfo>,
    #[serde(default = "default_content")]
    pub thursday: Vec<AnimeInfo>,
    #[serde(default = "default_content")]
    pub friday: Vec<AnimeInfo>,
    #[serde(default = "default_content")]
    pub saturday: Vec<AnimeInfo>,
    #[serde(default = "default_content")]
    pub sunday: Vec<AnimeInfo>,
    #[serde(default = "default_content")]
    pub other: Vec<AnimeInfo>,
    #[serde(default = "default_content")]
    pub unknown: Vec<AnimeInfo>,
}

fn default_content() -> Vec<AnimeInfo> {
    Vec::with_capacity(0)
}
