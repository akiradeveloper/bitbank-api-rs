use super::*;

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Ticker {
    #[serde_as(as = "DisplayFromStr")]
    pub pair: Pair,
    #[serde_as(deserialize_as = "DefaultOnNull<DisplayFromStr>")]
    pub sell: f64,
    #[serde_as(deserialize_as = "DefaultOnNull<DisplayFromStr>")]
    pub buy: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub high: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub low: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub open: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub last: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub vol: f64,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub timestamp: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
struct Response(Vec<Ticker>);

#[derive(TypedBuilder)]
pub struct Params {}

pub async fn get(_: Params) -> anyhow::Result<Vec<Ticker>> {
    let resp: Response = do_get("/tickers".to_owned()).await?;
    Ok(resp.0)
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
