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
