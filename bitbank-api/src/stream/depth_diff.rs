use super::*;

pub use crate::public::depth::Order;
use crate::public::depth::RawOrder;

#[serde_as]
#[derive(Deserialize, Debug)]
struct RawResponse {
    a: Vec<RawOrder>,
    b: Vec<RawOrder>,
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

#[derive(TypedBuilder)]
pub struct Params {
    pair: Pair,
}

pub async fn connect(
    params: Params,
) -> anyhow::Result<impl tokio_stream::Stream<Item = DepthDiff>> {
    use tokio_stream::StreamExt;

    let pair = params.pair;
    let room_id = format!("depth_diff_{pair}");
    let raw = do_connect::<RawResponse>(&room_id).await?;
    let st = raw.map(|x| DepthDiff {
        asks: x.a.into_iter().map(Order::new).collect(),
        bids: x.b.into_iter().map(Order::new).collect(),
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
