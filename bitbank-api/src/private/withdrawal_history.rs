use futures::sink::With;

use super::*;

#[derive(strum::EnumString, strum::Display, Debug)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Confirming,
    Examining,
    Sending,
    Done,
    Rejected,
    Canceled,
    ConfirmTimeout,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Withdrawal {
    pub uuid: String,
    #[serde_as(as = "DisplayFromStr")]
    pub asset: Asset,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub fee: f64,
    /// Exists only for crytos.
    pub txid: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    pub status: Status,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub requested_at: NaiveDateTime,
}

#[derive(Deserialize)]
struct Response {
    withdrawals: Vec<Withdrawal>,
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

pub async fn get(cred: Credential, params: Params) -> anyhow::Result<Vec<Withdrawal>> {
    let resp: Response = ApiExec { cred }
        .get("/v1/user/withdrawal_history", to_query_params(params)?)
        .await?;
    Ok(resp.withdrawals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test_get() {
        dotenv().ok();
        let cred = Credential::from_env().unwrap();
        let resp = get(cred, Params::builder().asset(Asset::XRP).count(10).build())
            .await
            .unwrap();
        dbg!(&resp);
    }
}
