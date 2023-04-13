use super::*;

#[derive(Deserialize)]
pub struct Response {
    pub trades: Vec<Trade>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Trade {
    pub trade_id: u64,
    pub order_id: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub pair: Pair,
    #[serde_as(as = "DisplayFromStr")]
    pub side: Side,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
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

pub async fn get(cred: Credential, params: Params) -> anyhow::Result<Vec<Trade>> {
    let resp: Response = ApiExec { cred }
        .get("/v1/user/spot/trade_history", params.to_query_params())
        .await?;
    Ok(resp.trades)
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
            .pair(Pair(XRP, JPY))
            .order(SortOrder::Asc)
            .build()?;
        assert_eq!(params.to_query_params(), "?pair=xrp_jpy&order=asc");
        Ok(())
    }
}
