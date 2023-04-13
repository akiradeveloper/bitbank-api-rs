use super::*;

#[serde_as]
#[serde_with::skip_serializing_none]
#[derive(Builder, Debug, serde::Serialize)]
#[builder(setter(strip_option, into))]
pub struct Params {
    #[serde_as(as = "DisplayFromStr")]
    pair: Pair,
    #[serde_as(as = "DisplayFromStr")]
    amount: f64,
    #[builder(default)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    price: Option<f64>,
    side: Side,
    #[serde(rename = "type")]
    order_type: OrderType,
    #[builder(default)]
    post_only: Option<bool>,
    #[builder(default)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    trigger_price: Option<f64>,
}

#[serde_as]
#[derive(serde::Deserialize, Debug)]
pub struct CreateOrder {
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
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub trigger_price: Option<f64>,
    pub status: OrderStatus,
}

pub async fn post(cred: Credential, params: Params) -> anyhow::Result<CreateOrder> {
    let json = serde_json::to_string(&params)?;
    ApiExec { cred }.post("/v1/user/spot/order", json).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params() -> anyhow::Result<()> {
        let p = ParamsBuilder::default()
            .pair(Pair(xrp, jpy))
            .price(50.2)
            .amount(10.1)
            .side(Side::buy)
            .order_type(OrderType::stop)
            .build()?;
        dbg!(&p);
        let json = serde_json::to_string(&p)?;
        dbg!(&json);
        Ok(())
    }
}
