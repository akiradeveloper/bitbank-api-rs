use super::*;

#[derive(Deserialize, Debug)]
struct RawResponse {
    asks: Vec<RawOrder>,
    bids: Vec<RawOrder>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub(crate) struct RawOrder(
    #[serde_as(as = "DisplayFromStr")] f64,
    #[serde_as(as = "DisplayFromStr")] f64,
);

#[derive(Debug)]
pub struct Order {
    pub price: f64,
    pub amount: f64,
}
impl Order {
    pub(crate) fn new(x: RawOrder) -> Self {
        Self {
            price: x.0,
            amount: x.1,
        }
    }
}

#[derive(Debug)]
pub struct Depth {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
}

#[derive(TypedBuilder)]
pub struct Params {
    pair: Pair,
}

pub async fn get(params: Params) -> anyhow::Result<Depth> {
    let path = format!("/{}/depth", params.pair);
    let resp: RawResponse = do_get(path).await?;
    Ok(Depth {
        asks: resp.asks.into_iter().map(Order::new).collect(),
        bids: resp.bids.into_iter().map(Order::new).collect(),
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
