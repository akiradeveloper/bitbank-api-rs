use super::*;

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Ticker {
    #[serde_as(as = "DisplayFromStr")]
    pub sell: f64,
    #[serde_as(as = "DisplayFromStr")]
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

#[derive(Builder)]
#[builder(setter(into))]
pub struct Params {
    pair: Pair,
}

pub async fn get(params: Params) -> anyhow::Result<Ticker> {
    let pair = params.pair;
    let path = format!("/{}/ticker", pair);
    do_get(path).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let params = ParamsBuilder::default().pair(Pair(XRP, JPY)).build()?;
        let _ = get(params).await?;
        Ok(())
    }
}
