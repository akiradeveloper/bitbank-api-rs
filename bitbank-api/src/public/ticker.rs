use super::*;

#[serde_as]
#[derive(Debug, Deserialize)]
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

pub async fn get(pair: Pair) -> anyhow::Result<Ticker> {
    let path = format!("/{}/ticker", pair);
    do_get(path).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let x = get(Pair(xrp, jpy)).await?;
        Ok(())
    }
}
