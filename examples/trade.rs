use bitbank_api::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const XRP_JPY: Pair = Pair(Asset::XRP, Asset::JPY);

    // For not actually complete a buy,
    // we will make a buy order in very low price.
    let ticker = {
        use public::ticker::*;
        let params = Params::builder().pair(XRP_JPY).build();
        get(params).await?
    };
    let tgt_price = ticker.low * 0.1;
    // Limit the accidental buy at 10 yen.
    let tgt_amount = 10.0 / tgt_price;
    dbg!(tgt_price, tgt_amount);

    let cred = private::Credential::from_env()?;
    dbg!(&cred);

    eprintln!("create order");
    let created = {
        use private::create_order::*;
        let params = Params::builder()
            .pair(XRP_JPY)
            .side(Side::Buy)
            .price(tgt_price)
            .amount(tgt_amount)
            .order_type(OrderType::Limit)
            .build();
        post(cred.clone(), params).await?
    };
    dbg!(&created);

    eprintln!("fetch order");
    let resp = {
        use private::fetch_order::*;
        let params = Params::builder()
            .pair(XRP_JPY)
            .order_id(created.order_id)
            .build();
        get(cred.clone(), params).await?
    };
    dbg!(&resp);

    eprintln!("cancel order");
    let resp = {
        use private::cancel_order::*;
        let params = Params::builder()
            .pair(XRP_JPY)
            .order_id(created.order_id)
            .build();
        post(cred.clone(), params).await?
    };
    dbg!(&resp);

    eprintln!("---- multiple orders ----");

    const N: usize = 3;
    eprintln!("create orders");
    let created = {
        use private::create_order::*;
        let mut out = vec![];
        for _ in 0..N {
            let params = Params::builder()
                .pair(XRP_JPY)
                .side(Side::Buy)
                .price(tgt_price)
                .amount(tgt_amount)
                .order_type(OrderType::Limit)
                .build();
            let x = post(cred.clone(), params).await?;
            out.push(x);
        }
        out
    };
    dbg!(&created);

    let mut order_ids = vec![];
    for i in 0..N {
        order_ids.push(created[i].order_id);
    }

    eprintln!("fetch orders");
    let resp = {
        use private::fetch_orders::*;
        let params = Params::builder()
            .pair(XRP_JPY)
            .order_ids(order_ids.clone())
            .build();
        post(cred.clone(), params).await?
    };
    dbg!(&resp);

    eprintln!("fetch active orders");
    let resp = {
        use private::fetch_active_orders::*;
        let params = Params::builder().pair(XRP_JPY).build();
        get(cred.clone(), params).await?
    };
    dbg!(&resp);

    eprintln!("cancel orders");
    let resp = {
        use private::cancel_orders::*;
        let params = Params::builder()
            .pair(XRP_JPY)
            .order_ids(order_ids.clone())
            .build();
        post(cred.clone(), params).await?
    };
    dbg!(&resp);

    Ok(())
}
