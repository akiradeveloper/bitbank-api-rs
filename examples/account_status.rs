use bitbank_api::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cred = private::Credential::from_env()?;
    dbg!(&cred);

    let resp = {
        let params = private::trade_history::Params::builder()
            .pair(Pair(Asset::XRP, Asset::JPY))
            .build();
        private::trade_history::get(cred.clone(), params).await?
    };
    dbg!(&resp);

    Ok(())
}
