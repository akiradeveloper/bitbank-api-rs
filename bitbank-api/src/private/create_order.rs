use super::*;

#[serde_as]
#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Debug)]
pub struct Params {
    #[serde_as(as = "DisplayFromStr")]
    pair: Pair,
    #[serde_as(as = "DisplayFromStr")]
    amount: f64,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[builder(default, setter(strip_option))]
    price: Option<f64>,
    #[serde_as(as = "DisplayFromStr")]
    side: Side,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "type")]
    order_type: OrderType,
    #[builder(default, setter(strip_option))]
    post_only: Option<bool>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[builder(default, setter(strip_option))]
    trigger_price: Option<f64>,
}

pub async fn post(cred: Credential, params: Params) -> anyhow::Result<OrderInfo> {
    let json = serde_json::to_string(&params)?;
    ApiExec { cred }.post("/v1/user/spot/order", json).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params() -> anyhow::Result<()> {
        let p = Params::builder()
            .pair(Pair(XRP, JPY))
            .price(50.2)
            .amount(10.1)
            .side(Side::Buy)
            .order_type(OrderType::Stop)
            .build();
        dbg!(&p);
        let json = serde_json::to_string(&p)?;
        dbg!(&json);
        Ok(())
    }
}
