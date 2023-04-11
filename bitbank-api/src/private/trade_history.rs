use super::*;

#[derive(serde::Deserialize)]
pub struct Response {
    pub trades: Vec<Trade>,
}

#[serde_as]
#[derive(serde::Deserialize, Debug)]
pub struct Trade {
    pub trade_id: u64,
    pub order_id: u64,
    pub pair: Pair,
    pub side: Side,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    pub maker_taker: MakerTaker,
    #[serde_as(as = "DisplayFromStr")]
    pub fee_amount_base: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub fee_amount_quote: f64,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub executed_at: NaiveDateTime,
}

#[derive(Builder, QueryParams, Debug)]
#[builder(setter(strip_option, into))]
pub struct Params {
    pair: Pair,
    #[builder(default)]
    count: Option<u16>,
    #[builder(default)]
    order_id: Option<u64>,
    #[builder(default)]
    since: Option<NaiveDateTime>,
    #[builder(default)]
    end: Option<NaiveDateTime>,
    #[builder(default)]
    order: Option<SortOrder>,
}

pub async fn get(cred: Credential, params: Params) -> anyhow::Result<Response> {
    ApiExec { cred }
        .get("/v1/user/spot/trade_history", params.to_query_params())
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_serde() -> anyhow::Result<()> {
        let s = r#"
        {
            "trade_id": 123,
            "order_id": 456,
            "pair": "xrp_jpy",
            "side": "sell",
            "price": "589",
            "type": "market",
            "amount": "1",
            "maker_taker": "maker",
            "fee_amount_base": "1",
            "fee_amount_quote": "2",
            "executed_at": 123456
        }
        "#;
        dbg!(&s);

        let v: Trade = serde_json::from_str(&s)?;
        dbg!(&v);
        Ok(())
    }
    #[test]
    fn test_params() -> anyhow::Result<()> {
        let params = ParamsBuilder::default()
            .pair(Pair::xrp_jpy)
            .order(SortOrder::asc)
            .build()?;
        assert_eq!(params.to_query_params(), "?pair=xrp_jpy&order=asc");
        Ok(())
    }
}
