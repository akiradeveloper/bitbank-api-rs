use bitbank_api::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cred = private::Credential::from_env()?;
    dbg!(&cred);

    let resp = {
        let params = private::trade_history::ParamsBuilder::default()
            .pair(Pair(Asset::xrp, Asset::jpy))
            .build()?;
        private::trade_history::get(cred.clone(), params).await?
    };
    dbg!(&resp.trades);

    Ok(())
}
