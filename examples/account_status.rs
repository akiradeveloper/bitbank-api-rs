use bitbank_api::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let cred = private::Credential::from_env()?;

    eprintln!("deposit history");
    let resp = {
        use private::deposit_history::*;
        let params = Params::builder().asset(Asset::XRP).count(3).build();
        get(cred.clone(), params).await?
    };
    dbg!(&resp);

    eprintln!("withdrawal history");
    let resp = {
        use private::withdrawal_history::*;
        let params = Params::builder().asset(Asset::XRP).count(3).build();
        get(cred.clone(), params).await?
    };
    dbg!(&resp);

    eprintln!("trade history");
    let resp = {
        use private::trade_history::*;
        let params = Params::builder()
            .pair(Pair(Asset::XRP, Asset::JPY))
            .count(3)
            .build();
        get(cred.clone(), params).await?
    };
    dbg!(&resp);

    eprintln!("assets");
    let resp = {
        use private::assets::*;
        let params = Params::builder().build();
        get(cred.clone(), params).await?
    };
    dbg!(&resp);

    Ok(())
}
