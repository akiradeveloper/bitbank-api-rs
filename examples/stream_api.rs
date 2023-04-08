use futures_util::{pin_mut, StreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let st = bitbank_api::stream::ticker::connect(bitbank_api::Pair::xrp_jpy).await?;
    pin_mut!(st);
    while let Some(ticker) = st.next().await {
        dbg!(ticker);
    }
    Ok(())
}
