use super::*;

#[derive(Deserialize)]
struct Response {
    orders: Vec<OrderInfo>,
}

#[serde_as]
#[derive(TypedBuilder, Serialize, Debug)]
pub struct Params {
    #[serde_as(as = "DisplayFromStr")]
    pair: Pair,
    #[builder(default, setter(strip_option))]
    count: Option<u64>,
    #[builder(default, setter(strip_option))]
    from_id: Option<u64>,
    #[builder(default, setter(strip_option))]
    end_id: Option<u64>,
    #[builder(default, setter(strip_option))]
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    since: Option<NaiveDateTime>,
    #[builder(default, setter(strip_option))]
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    end: Option<NaiveDateTime>,
}

pub async fn get(cred: Credential, params: Params) -> anyhow::Result<Vec<OrderInfo>> {
    let resp: Response = ApiExec { cred }
        .get("/v1/user/spot/active_orders", to_query_params(params)?)
        .await?;
    Ok(resp.orders)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params() -> anyhow::Result<()> {
        let params = Params::builder()
            .pair(Pair(XRP, JPY))
            .since(NaiveDateTime::from_timestamp_opt(10, 0).unwrap())
            .count(100)
            .build();
        assert_eq!(
            to_query_params(params)?,
            "pair=xrp_jpy&count=100&since=10000"
        );
        Ok(())
    }
}
