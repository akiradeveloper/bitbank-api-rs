#[cfg(test)]
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_urlencoded::to_string as to_query_params;
use serde_with::chrono::NaiveDateTime;
use serde_with::DefaultOnNull;
use serde_with::TimestampMilliSeconds;
use typed_builder::TypedBuilder;

/// Private API with authentication.
pub mod private;
/// Public API without authentication.
pub mod public;
/// Public streaming API.
pub mod stream;

use serde_with::{serde_as, DisplayFromStr};

/// Asset pair
/// - 0: base asset
/// - 1: quote asset
#[derive(derive_more::Display, Debug, Clone)]
#[display(fmt = "{_0}_{_1}")]
pub struct Pair(pub Asset, pub Asset);

impl std::str::FromStr for Pair {
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

/// Asset type
#[derive(strum::EnumString, strum::Display, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Asset {
    JPY,
    BTC,
    XRP,
    LTC,
    ETH,
    MONA,
    BCC,
    XLM,
    QTUM,
    BAT,
    OMG,
    XYM,
    LINK,
    MKR,
    BOBA,
    ENJ,
    MATIC,
    DOT,
    DOGE,
    ASTR,
    ADA,
    AVAX,
    AXS,
    FLR,
    SAND,
    GALA,
    APE,
    CHZ,
    OAS,
    MANA,
    GRT,
    RNDR,
    BNB,
    DAI,
    OP,
    ARB,
    KLAY,
    IMX,
    MASK,
    POL,
    SOL,
    CYBER,
    RENDER,
    TRX,
}
#[cfg(test)]
pub use Asset::*;

/// desc or asc
#[derive(strum::EnumString, strum::Display, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum SortOrder {
    Desc,
    Asc,
}

/// buy or sell
#[derive(strum::EnumString, strum::Display, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
}

/// limit or market or stop or stop limit
#[derive(strum::EnumString, strum::Display, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopLimit,
}

/// maker or taker
#[derive(strum::EnumString, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum MakerTaker {
    Maker,
    Taker,
}

/// Status of order
#[derive(strum::EnumString, Debug, Clone)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Inactive,
    Unfilled,
    ParitallyFilled,
    FullyFilled,
    CanceledUnfilled,
    CanceledPartiallyFilled,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
struct ResponseError {
    code: u16,
}
