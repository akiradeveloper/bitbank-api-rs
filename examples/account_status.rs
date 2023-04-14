use bitbank_api::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cred = private::Credential::from_env()?;
    dbg!(&cred);

    let resp = {
        use private::trade_history::*;
        let params = Params::builder().pair(Pair(Asset::XRP, Asset::JPY)).build();
        get(cred.clone(), params).await?
    };
    dbg!(&resp);

    Ok(())
}
