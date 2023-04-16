use super::*;

#[serde_as]
#[derive(TypedBuilder, Serialize, Debug)]
pub struct Params {
    #[serde_as(as = "DisplayFromStr")]
    pair: Pair,
    order_id: u64,
}

pub async fn post(cred: Credential, params: Params) -> anyhow::Result<OrderInfo> {
    let json = serde_json::to_string(&params)?;
    ApiExec { cred }
        .post("/v1/user/spot/cancel_order", json)
        .await
}
