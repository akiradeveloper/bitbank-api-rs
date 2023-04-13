use derive_builder::Builder;
use derive_more::Display;
use query_params::QueryParams;
use serde::Deserialize;
use serde_with::chrono::NaiveDateTime;
use serde_with::TimestampMilliSeconds;
use std::str::FromStr;

pub mod private;
pub mod public;
pub mod stream;

use serde_with::{serde_as, DisplayFromStr};

/// Asset pair
/// - 0: base asset
/// - 1: quote asset
#[derive(Display, Debug, Clone)]
#[display(fmt = "{_0}_{_1}")]
pub struct Pair(pub Asset, pub Asset);

impl FromStr for Pair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Find the _ between the pair.
        let sep = s
            .char_indices()
            .find(|(_, c)| *c == '_')
            .map(|(idx, _)| idx)
            .ok_or(anyhow::anyhow!("failed to parse pair"))?;

        let x = s[0..sep].parse()?;
        let y = s[sep + 1..].parse()?;

        Ok(Self(x, y))
    }
}

#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumString)]
pub enum Asset {
    jpy,
    btc,
    xrp,
    ltc,
    eth,
    mona,
    bcc,
    xlm,
    qtum,
    bat,
    omg,
    xym,
    link,
    mkr,
    boba,
    enj,
    matic,
    dot,
    doge,
    astr,
    ada,
    avax,
    axs,
    flr,
    sand,
    gala,
    ape,
    chz,
    oas,
}
#[cfg(test)]
pub use Asset::*;

#[derive(Display, Debug, Clone)]
pub enum SortOrder {
    desc,
    asc,
}

#[derive(Display, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Side {
    buy,
    sell,
}

#[derive(Display, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum OrderType {
    limit,
    market,
    stop,
    stop_limit,
}

#[derive(Display, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum MakerTaker {
    maker,
    taker,
}

#[derive(Display, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum OrderStatus {
    INACTIVE,
    UNFILLED,
    PARTIALLY_FILLED,
    FULLY_FILLED,
    CANCELED_UNFILLED,
    CANCELED_PARTIALLY_FILLED,
}

#[derive(Debug, serde::Deserialize)]
struct Response {
    success: u16,
    data: serde_json::Value,
}

impl Response {
    fn result(self) -> anyhow::Result<serde_json::Value> {
        if self.success == 1 {
            Ok(self.data)
        } else {
            let e: ResponseError = serde_json::from_value(self.data)?;
            Err(ApiError { code: e.code }.into())
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("bitbank API error (code={code})")]
struct ApiError {
    code: u16,
}

#[derive(Debug, serde::Deserialize)]
struct ResponseError {
    code: u16,
}
