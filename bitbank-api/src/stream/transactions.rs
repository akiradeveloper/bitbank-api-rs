use super::*;

pub use crate::public::transactions::Transaction;

#[derive(TypedBuilder)]
pub struct Params {
    pair: Pair,
}

#[derive(Deserialize, Debug)]
pub struct Transactions {
    pub transactions: Vec<Transaction>,
}

pub async fn connect(
    params: Params,
) -> anyhow::Result<impl tokio_stream::Stream<Item = Transaction>> {
    use futures::stream::{self, StreamExt};
    let pair = params.pair;
    let room_id = format!("transactions_{pair}");
    let st = do_connect::<Transactions>(&room_id).await?;
    let st = st.flat_map(|x| stream::iter(x.transactions));
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
            break;
        }
        Ok(())
    }
}
