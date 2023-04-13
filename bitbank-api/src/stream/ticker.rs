use super::*;

pub use crate::public::ticker::Ticker;

#[derive(TypedBuilder)]
pub struct Params {
    pair: Pair,
}

pub async fn connect(params: Params) -> anyhow::Result<impl tokio_stream::Stream<Item = Ticker>> {
    let pair = params.pair;
    let room_id = format!("ticker_{pair}");
    do_connect(&room_id).await
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
        let mut n = 0;
        while let Some(_) = st.next().await {
            n += 1;
            if n >= 5 {
                break;
            }
        }
        Ok(())
    }
}
