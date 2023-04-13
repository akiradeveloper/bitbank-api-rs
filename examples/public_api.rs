use bitbank_api::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ticker = public::ticker::get(Pair(Asset::xrp, Asset::jpy)).await?;
    dbg!(ticker);
    Ok(())
}
