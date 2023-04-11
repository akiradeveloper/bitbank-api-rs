use super::*;

#[derive(Builder, QueryParams, Debug)]
#[builder(setter(strip_option, into))]
pub struct Params {
    pair: Pair,
    order_id: u64,
}

#[derive(serde::Deserialize, Debug)]
#[serde_as]
pub struct OrderInfo {
    pub order_id: u64,
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
    pub price: f64,
    pub post_only: bool,
    #[serde_as(as = "DisplayFromStr")]
    pub average_price: f64,
    pub ordered_at: NaiveDateTime,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub expired_at: Option<NaiveDateTime>,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub triggered_at: Option<NaiveDateTime>,
    #[serde_as(as = "DisplayFromStr")]
    pub trigger_price: f64,
    pub status: OrderStatus,
}

pub async fn get(cred: Credential, params: Params) -> anyhow::Result<OrderInfo> {
    ApiExec { cred }
        .get("/v1/user/spot/order", params.to_query_params())
        .await
}
