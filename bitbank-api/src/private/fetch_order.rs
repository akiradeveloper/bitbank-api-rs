use super::*;

#[derive(TypedBuilder, QueryParams, Debug)]
pub struct Params {
    pair: Pair,
    order_id: u64,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct OrderInfo {
    pub order_id: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub pair: Pair,
    #[serde_as(as = "DisplayFromStr")]
    pub side: Side,
    #[serde_as(as = "DisplayFromStr")]
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
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub triggered_at: Option<NaiveDateTime>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub trigger_price: Option<f64>,
    #[serde_as(as = "DisplayFromStr")]
    pub status: OrderStatus,
}

pub async fn get(cred: Credential, params: Params) -> anyhow::Result<OrderInfo> {
    ApiExec { cred }
        .get("/v1/user/spot/order", params.to_query_params())
        .await
}
