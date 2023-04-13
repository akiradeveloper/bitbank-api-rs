use super::*;

pub use crate::public::ticker::Ticker;

pub async fn connect(pair: Pair) -> anyhow::Result<impl tokio_stream::Stream<Item = Ticker>> {
    let room_id = format!("ticker_{pair}");
    do_connect(&room_id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        use futures_util::{pin_mut, StreamExt};

        let mut n = 0;
        let st = connect(Pair(xrp, jpy)).await?;
        pin_mut!(st);
        while let Some(x) = st.next().await {
            n += 1;
            if n >= 5 {
                break;
            }
        }
        Ok(())
    }
}
