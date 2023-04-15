use super::*;

#[derive(Deserialize, Debug)]
struct Response {
    transactions: Vec<Transaction>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub transaction_id: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub side: Side,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: f64,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub executed_at: NaiveDateTime,
}

#[derive(derive_more::Display)]
#[display(fmt = "{_0}{_1}{_2}")]
pub struct Date(u16, u8, u8);

#[derive(TypedBuilder)]
pub struct Params {
    pair: Pair,
    #[builder(default, setter(strip_option))]
    date: Option<Date>,
}

pub async fn get(params: Params) -> anyhow::Result<Vec<Transaction>> {
    let pair = params.pair;
    let date = params
        .date
        .map(|x| format!("/{x}"))
        .unwrap_or("".to_owned());

    let path = format!("/{pair}/transactions{date}");
    let resp: Response = do_get(path).await?;
    Ok(resp.transactions)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_transactions_no_date() -> anyhow::Result<()> {
        let params = Params::builder().pair(Pair(XRP, JPY)).build();
        let resp = get(params).await?;
        assert!(resp.len() <= 60);
        dbg!(&resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_transactions() -> anyhow::Result<()> {
        let params = Params::builder()
            .pair(Pair(XRP, JPY))
            .date(Date(2022, 12, 25))
            .build();
        let resp = get(params).await?;
        dbg!(&resp);
        Ok(())
    }
}
