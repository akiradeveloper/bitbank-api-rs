use bitbank_api::*;
use futures_util::{pin_mut, StreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use stream::ticker::*;
    let params = Params::builder().pair(Pair(Asset::XRP, Asset::JPY)).build();
    let st = connect(params).await?;
    pin_mut!(st);
    while let Some(ticker) = st.next().await {
        dbg!(ticker);
    }
    Ok(())
}
