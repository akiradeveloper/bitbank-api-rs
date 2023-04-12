use derive_more::Display;
use serde::Deserialize;
use serde_with::chrono::NaiveDateTime;
use serde_with::TimestampMilliSeconds;

pub mod private;
pub mod public;
pub mod stream;

use serde_with::{serde_as, DisplayFromStr};

#[derive(Display, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Pair {
    xrp_jpy,
}

#[derive(Display)]
pub enum Asset {
    jpy,
    xrp,
}

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
