use super::*;

#[derive(strum::EnumString, strum::Display, Debug)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Found,
    Confirmed,
    Done,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Deposit {
    pub uuid: String,
    #[serde_as(as = "DisplayFromStr")]
    pub asset: Asset,
    pub amount: f64,
    /// Exists only for crytos.
    pub txid: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    pub status: Status,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub found_at: NaiveDateTime,
    /// Exists only for confirmed ones.
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub confirmed_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
struct Response {
    deposits: Vec<Deposit>,
}

#[serde_as]
#[derive(TypedBuilder, Serialize, Debug)]
pub struct Params {
    #[serde_as(as = "DisplayFromStr")]
    asset: Asset,
    #[builder(default, setter(strip_option))]
    count: Option<u8>,
    #[builder(default, setter(strip_option))]
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    since: Option<NaiveDateTime>,
    #[builder(default, setter(strip_option))]
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    end: Option<NaiveDateTime>,
}

pub async fn get(cred: Credential, params: Params) -> anyhow::Result<Vec<Deposit>> {
    let resp: Response = ApiExec { cred }
        .get("/v1/user/spot/deposit_history", to_query_params(params)?)
        .await?;
    Ok(resp.deposits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params() -> anyhow::Result<()> {
        let params = Params::builder()
            .asset(XRP)
            .since(NaiveDateTime::from_timestamp_opt(10, 0).unwrap())
            .count(100)
            .build();
        assert_eq!(to_query_params(params)?, "asset=xrp&count=100&since=10000");
        Ok(())
    }
}
