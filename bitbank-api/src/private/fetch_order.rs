use super::*;

#[serde_as]
#[derive(TypedBuilder, Serialize, Debug)]
pub struct Params {
    #[serde_as(as = "DisplayFromStr")]
    pair: Pair,
    order_id: u64,
}

pub async fn get(cred: Credential, params: Params) -> anyhow::Result<OrderInfo> {
    ApiExec { cred }
        .get("/v1/user/spot/order", to_query_params(params)?)
        .await
}
