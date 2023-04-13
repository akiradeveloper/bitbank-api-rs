use super::*;

#[derive(strum::EnumString, strum::Display, Debug)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Normal,
    Busy,
    VeryBusy,
    Halt,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct ExchangeStatus {
    #[serde_as(as = "DisplayFromStr")]
    pub pair: Pair,
    #[serde_as(as = "DisplayFromStr")]
    pub status: Status,
    #[serde_as(as = "DisplayFromStr")]
    pub min_amount: f64,
}

#[derive(Deserialize, Debug)]
struct Response {
    statuses: Vec<ExchangeStatus>,
}

// For design consistency sake, empty params should be given.
#[derive(Builder)]
#[builder(setter(into))]
pub struct Params {}

pub async fn get(_: Params) -> anyhow::Result<Vec<ExchangeStatus>> {
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
        let params = ParamsBuilder::default().build()?;
        let resp = get(params).await?;
        dbg!(&resp);
        Ok(())
    }
}
