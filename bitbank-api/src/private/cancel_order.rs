use super::*;

#[serde_as]
#[derive(Builder, Debug, serde::Serialize)]
#[builder(setter(strip_option, into))]
pub struct Params {
    #[serde_as(as = "DisplayFromStr")]
    pair: Pair,
    order_id: u64,
}

#[serde_as]
#[derive(serde::Deserialize, Debug)]
pub struct CancelOrder {
    pub order_id: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub pair: Pair,
    pub side: Side,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde_as(as = "DisplayFromStr")]
    pub start_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub remaining_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub executed_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    pub post_only: bool,
    #[serde_as(as = "DisplayFromStr")]
    pub average_price: f64,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub ordered_at: NaiveDateTime,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub expire_at: Option<NaiveDateTime>,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub canceled_at: NaiveDateTime,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub triggered_at: Option<NaiveDateTime>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub trigger_price: Option<f64>,
    pub status: OrderStatus,
}

pub async fn post(cred: Credential, params: Params) -> anyhow::Result<CancelOrder> {
    let json = serde_json::to_string(&params)?;
    ApiExec { cred }
        .post("/v1/user/spot/cancel_order", json)
        .await
}
