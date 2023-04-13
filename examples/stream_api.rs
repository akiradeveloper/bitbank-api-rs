use bitbank_api::*;
use futures_util::{pin_mut, StreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let st = stream::ticker::connect(Pair(Asset::xrp, Asset::jpy)).await?;
    pin_mut!(st);
    while let Some(ticker) = st.next().await {
        dbg!(ticker);
    }
    Ok(())
}
