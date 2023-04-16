use super::*;

pub use super::ticker::Ticker;

#[serde_as]
#[derive(Deserialize, Debug)]
struct Raw {
    #[serde_as(as = "DisplayFromStr")]
    pub pair: Pair,
    #[serde(flatten)]
    pub inner: crate::public::ticker::Ticker,
}

#[derive(Deserialize, Debug)]
struct Response(Vec<Raw>);

#[derive(TypedBuilder)]
pub struct Params {}

pub async fn get(_: Params) -> anyhow::Result<Vec<(Pair, Ticker)>> {
    let resp: Response = do_get("/tickers".to_owned()).await?;
    let out = resp.0.into_iter().map(|x| (x.pair, x.inner)).collect();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let params = Params::builder().build();
        let x = get(params).await?;
        dbg!(&x);
        Ok(())
    }
}
