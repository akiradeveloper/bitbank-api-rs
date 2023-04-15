use super::*;

#[derive(Deserialize, Debug)]
struct Response {
    asks: Vec<serde_json::Value>,
    bids: Vec<serde_json::Value>,
}

#[derive(Debug)]
pub struct Order {
    pub price: f64,
    pub amount: f64,
}

impl Order {
    pub fn from_json_value(mut value: serde_json::Value) -> Option<Self> {
        let arr = value.as_array_mut().unwrap();
        arr.reverse();
        let mut out = [0.; 2];
        for i in 0..2 {
            let v: f64 = arr.pop()?.as_str()?.parse().ok()?;
            out[i] = v;
        }
        Some(Self {
            price: out[0],
            amount: out[1],
        })
    }
}

#[derive(Default, Debug)]
pub struct Depth {
    asks: Vec<Order>,
    bids: Vec<Order>,
}

#[derive(TypedBuilder)]
pub struct Params {
    pair: Pair,
}

pub async fn get(params: Params) -> anyhow::Result<Depth> {
    let path = format!("/{}/depth", params.pair);
    let resp: Response = do_get(path).await?;
    let mut out = Depth::default();
    for x in resp.asks {
        let y =
            Order::from_json_value(x).ok_or(anyhow::anyhow!("failed to parse a depth order"))?;
        out.asks.push(y);
    }
    for x in resp.bids {
        let y =
            Order::from_json_value(x).ok_or(anyhow::anyhow!("failed to parse a depth order"))?;
        out.bids.push(y);
    }
    Ok(out)
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
