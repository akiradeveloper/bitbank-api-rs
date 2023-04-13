use super::*;

#[derive(serde::Deserialize, Debug)]
pub enum Status {
    NORMAL,
    BUSY,
    VERY_BUSY,
    HALT,
}

#[serde_as]
#[derive(serde::Deserialize, Debug)]
pub struct ExchangeStatus {
    pair: Pair,
    status: Status,
    #[serde_as(as = "DisplayFromStr")]
    min_amount: f64,
}

#[derive(serde::Deserialize, Debug)]
struct Response {
    statuses: Vec<ExchangeStatus>,
}

pub async fn get() -> anyhow::Result<Vec<ExchangeStatus>> {
    let path = "/v1/spot/status";
    let params = "".to_owned();
    let resp: Response = do_get_private(path, params).await?;
    Ok(resp.statuses)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_exchange_status() -> anyhow::Result<()> {
        let resp = get().await?;
        dbg!(&resp);
        Ok(())
    }
}
