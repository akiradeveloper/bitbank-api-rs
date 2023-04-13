use bitbank_api::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // For not actually complete a buy,
    // we will make a buy order in non-realistically low price.
    let ticker = public::ticker::get(Pair(Asset::xrp, Asset::jpy)).await?;
    let tgt_price = ticker.low * 0.1;

    let cred = private::Credential::from_env()?;
    dbg!(&cred);

    let created = {
        let params = private::create_order::ParamsBuilder::default()
            .pair(Pair(Asset::xrp, Asset::jpy))
            .side(Side::buy)
            .price(tgt_price)
            .amount(1)
            .order_type(OrderType::limit)
            .build()?;
        private::create_order::post(cred.clone(), params).await?
    };
    dbg!(&created);

    tokio::time::sleep(Duration::from_secs(1)).await;
    let fetched = {
        let params = private::fetch_order::ParamsBuilder::default()
            .pair(Pair(Asset::xrp, Asset::jpy))
            .order_id(created.order_id)
            .build()?;
        private::fetch_order::get(cred.clone(), params).await?
    };
    dbg!(&fetched);

    tokio::time::sleep(Duration::from_secs(1)).await;
    let canceled = {
        let params = private::cancel_order::ParamsBuilder::default()
            .pair(Pair(Asset::xrp, Asset::jpy))
            .order_id(created.order_id)
            .build()?;
        private::cancel_order::post(cred.clone(), params).await?
    };
    dbg!(&canceled);

    Ok(())
}
