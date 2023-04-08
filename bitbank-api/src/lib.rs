use derive_more::Display;
use serde::Deserialize;
use serde_with::chrono::NaiveDateTime;
use serde_with::TimestampMilliSeconds;

pub mod stream;

use serde_with::{serde_as, DisplayFromStr};

pub mod ticker;

#[derive(Display)]
pub enum Pair {
    btc_jpy,
    xrp_jpy,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Ticker {
    #[serde_as(as = "DisplayFromStr")]
    pub sell: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub buy: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub high: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub low: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub open: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub last: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub vol: f64,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, serde::Deserialize)]
struct Response {
    success: u16,
    data: serde_json::Value,
}

async fn do_get<R: serde::de::DeserializeOwned>(url: String) -> anyhow::Result<R> {
    let cli = reqwest::Client::new();
    let resp: Response = cli.get(url).send().await?.json().await?;
    anyhow::ensure!(resp.success == 1);
    let data: R = serde_json::from_value(resp.data)?;
    Ok(data)
}
