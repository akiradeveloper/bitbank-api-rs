use super::*;

pub use crate::public::depth::Order;
use crate::public::depth::RawOrder;

#[serde_as]
#[derive(Deserialize, Debug)]
struct RawResponse {
    asks: Vec<RawOrder>,
    bids: Vec<RawOrder>,
    #[serde_as(as = "TimestampMilliSeconds")]
    timestamp: NaiveDateTime,
    #[serde_as(as = "DisplayFromStr")]
    sequenceId: u64,
}

#[derive(TypedBuilder)]
pub struct Params {
    pair: Pair,
}

#[derive(Debug)]
pub struct DepthWhole {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
    pub timestamp: NaiveDateTime,
    pub sequence_id: u64,
}

pub async fn connect(
    params: Params,
) -> anyhow::Result<impl tokio_stream::Stream<Item = DepthWhole>> {
    use tokio_stream::StreamExt;

    let pair = params.pair;
    let room_id = format!("depth_whole_{pair}");
    let raw = do_connect::<RawResponse>(&room_id).await?;
    let st = raw.map(|x| DepthWhole {
        asks: x.asks.into_iter().map(Order::new).collect(),
        bids: x.bids.into_iter().map(Order::new).collect(),
        timestamp: x.timestamp,
        sequence_id: x.sequenceId,
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
            dbg!(x);
            break;
        }
        Ok(())
    }
}
