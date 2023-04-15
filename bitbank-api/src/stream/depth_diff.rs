use super::*;

pub use crate::public::depth::Order;

#[derive(TypedBuilder)]
pub struct Params {
    pair: Pair,
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct RawResponse {
    a: Vec<serde_json::Value>,
    b: Vec<serde_json::Value>,
    #[serde_as(as = "TimestampMilliSeconds")]
    t: NaiveDateTime,
    #[serde_as(as = "DisplayFromStr")]
    s: u64,
}

#[derive(Debug)]
pub struct DepthDiff {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
    pub timestamp: NaiveDateTime,
    pub sequence_id: u64,
}

fn conv_list(xs: Vec<serde_json::Value>) -> Vec<Order> {
    let mut out = vec![];
    for x in xs {
        let y = Order::from_json_value(x).unwrap();
        out.push(y);
    }
    out
}

pub async fn connect(
    params: Params,
) -> anyhow::Result<impl tokio_stream::Stream<Item = DepthDiff>> {
    use tokio_stream::StreamExt;

    let pair = params.pair;
    let room_id = format!("depth_diff_{pair}");
    let raw = do_connect::<RawResponse>(&room_id).await?;
    let st = raw.map(|x| DepthDiff {
        asks: conv_list(x.a), // TODO error handling
        bids: conv_list(x.b),
        timestamp: x.t,
        sequence_id: x.s,
    });
    Ok(st)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        use futures_util::{pin_mut, StreamExt};

        let params = Params::builder().pair(Pair(XRP, JPY)).build();
        let st = connect(params).await?;
        pin_mut!(st);
        while let Some(x) = st.next().await {
            dbg!(&x);
            break;
        }
        Ok(())
    }
}
