use bitbank_api::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // For not actually complete a buy,
    // we will make a buy order in non-realistically low price.
    let ticker = {
        use public::ticker::*;
        let params = Params::builder().pair(Pair(Asset::XRP, Asset::JPY)).build();
        get(params).await?
    };
    let tgt_price = ticker.low * 0.1;

    let cred = private::Credential::from_env()?;
    dbg!(&cred);

    let created = {
        use private::create_order::*;
        let params = Params::builder()
            .pair(Pair(Asset::XRP, Asset::JPY))
            .side(Side::Buy)
            .price(tgt_price)
            .amount(1.)
            .order_type(OrderType::Limit)
            .build();
        post(cred.clone(), params).await?
    };
    dbg!(&created);

    let fetched = {
        use private::fetch_order::*;
        let params = Params::builder()
            .pair(Pair(Asset::XRP, Asset::JPY))
            .order_id(created.order_id)
            .build();
        get(cred.clone(), params).await?
    };
    dbg!(&fetched);

    let canceled = {
        use private::cancel_order::*;
        let params = Params::builder()
            .pair(Pair(Asset::XRP, Asset::JPY))
            .order_id(created.order_id)
            .build();
        post(cred.clone(), params).await?
    };
    dbg!(&canceled);

    Ok(())
}
