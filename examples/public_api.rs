#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ticker = bitbank_api::public::ticker::get(bitbank_api::Pair::xrp_jpy).await?;
    dbg!(ticker);
    Ok(())
}
