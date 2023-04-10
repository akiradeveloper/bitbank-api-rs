use bitbank_api::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ticker = public::ticker::get(Pair::xrp_jpy).await?;

    // For not actually complete a buy,
    // we will make a buy order in non-realistic low price.
    let tgt_price = ticker.low * 0.1;

    let cred = private::Credential::from_env()?;

    let created = {
        let params = private::create_order::ParamsBuilder::default()
            .side(Side::buy)
            .pair(Pair::xrp_jpy)
            .price(tgt_price)
            .amount(1)
            .order_type(OrderType::market)
            .build()?;
        private::create_order::post(cred.clone(), params).await?
    };
    dbg!(&created);

    let fetched = {
        let params = private::fetch_order::ParamsBuilder::default()
            .pair(Pair::xrp_jpy)
            .order_id(created.order_id)
            .build()?;
        private::fetch_order::get(cred.clone(), params).await?
    };
    dbg!(&fetched);

    let canceled = {
        let params = private::cancel_order::ParamsBuilder::default()
            .pair(Pair::xrp_jpy)
            .order_id(created.order_id)
            .build()?;
        private::cancel_order::post(cred.clone(), params).await?
    };
    dbg!(&canceled);

    Ok(())
}
