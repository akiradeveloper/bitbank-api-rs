use super::*;

#[derive(Deserialize, Debug)]
struct RawResponse {
    asks: Vec<RawLimitOrder>,
    bids: Vec<RawLimitOrder>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub(crate) struct RawLimitOrder(
    #[serde_as(as = "DisplayFromStr")] f64,
    #[serde_as(as = "DisplayFromStr")] f64,
);

#[derive(Debug)]
pub struct LimitOrder {
    pub price: f64,
    pub amount: f64,
}
impl LimitOrder {
    pub(crate) fn new(x: RawLimitOrder) -> Self {
        Self {
            price: x.0,
            amount: x.1,
        }
    }
}

#[derive(Debug)]
pub struct Depth {
    pub asks: Vec<LimitOrder>,
    pub bids: Vec<LimitOrder>,
}

#[derive(TypedBuilder)]
pub struct Params {
    pair: Pair,
}

pub async fn get(params: Params) -> anyhow::Result<Depth> {
    let path = format!("/{}/depth", params.pair);
    let resp: RawResponse = do_get(path).await?;
    Ok(Depth {
        asks: resp.asks.into_iter().map(LimitOrder::new).collect(),
        bids: resp.bids.into_iter().map(LimitOrder::new).collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_depth() -> anyhow::Result<()> {
        let params = Params::builder().pair(Pair(XRP, JPY)).build();
        let resp = get(params).await?;
        dbg!(&resp);
        Ok(())
    }
}
