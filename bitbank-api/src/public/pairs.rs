use super::*;

#[derive(Deserialize, Debug)]
struct Response {
    pairs: Vec<PairInfo>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct PairInfo {
    #[serde_as(as = "DisplayFromStr")]
    pub name: Pair,
    #[serde_as(as = "DisplayFromStr")]
    pub base_asset: Asset,
    #[serde_as(as = "DisplayFromStr")]
    pub quote_asset: Asset,
    #[serde_as(as = "DisplayFromStr")]
    pub maker_fee_rate_base: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub taker_fee_rate_base: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub maker_fee_rate_quote: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub taker_fee_rate_quote: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub unit_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub limit_max_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub market_max_amount: f64,
    pub price_digits: u8,
    pub amount_digits: u8,
    pub is_enabled: bool,
    pub stop_order: bool,
    pub stop_order_and_cancel: bool,
    pub stop_market_order: bool,
    pub stop_stop_order: bool,
    pub stop_stop_limit_order: bool,
    pub stop_buy_order: bool,
    pub stop_sell_order: bool,
}

#[derive(TypedBuilder)]
pub struct Params {}

pub async fn get(_: Params) -> anyhow::Result<Vec<PairInfo>> {
    let path = "/v1/spot/pairs";
    let params = "".to_owned();
    let resp: Response = do_get_private(path, params).await?;
    Ok(resp.pairs)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_pairs() -> anyhow::Result<()> {
        let params = Params::builder().build();
        let resp = get(params).await?;
        dbg!(&resp);
        Ok(())
    }
}
