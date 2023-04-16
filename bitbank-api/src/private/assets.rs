use super::*;

#[derive(TypedBuilder)]
pub struct Params {}

#[serde_as]
#[derive(strum::EnumString, Deserialize, Debug)]
#[serde(untagged)]
pub enum WithdrawalFee {
    Crypto(#[serde_as(as = "DisplayFromStr")] f64),
    Fiat {
        #[serde_as(as = "DisplayFromStr")]
        under: f64,
        #[serde_as(as = "DisplayFromStr")]
        over: f64,
        #[serde_as(as = "DisplayFromStr")]
        threshold: f64,
    },
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct AssetInfo {
    #[serde_as(as = "DisplayFromStr")]
    pub asset: Asset,
    #[serde_as(as = "DisplayFromStr")]
    pub free_amount: f64,
    pub amount_precision: u8,
    #[serde_as(as = "DisplayFromStr")]
    pub onhand_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub locked_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub withdrawal_fee: WithdrawalFee,
    pub stop_deposit: bool,
    pub stop_withdrawal: bool,
}

#[derive(Deserialize)]
struct Response {
    assets: Vec<AssetInfo>,
}

pub async fn get(cred: Credential, _: Params) -> anyhow::Result<Vec<AssetInfo>> {
    let resp: Response = ApiExec { cred }
        .get("/v1/user/assets", "".to_owned())
        .await?;
    Ok(resp.assets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdrawal_deser() -> anyhow::Result<()> {
        let s = r#"
		"100.0"
		"#;
        let x: WithdrawalFee = serde_json::from_str(&s)?;
        dbg!(&x);

        let s = r#"
		{
            "under": "1.0",
            "over": "2.0",
            "threshold": "3.0"
        }
		"#;
        let x: WithdrawalFee = serde_json::from_str(&s)?;
        dbg!(&x);

        Ok(())
    }
}
