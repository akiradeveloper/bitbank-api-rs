use super::*;

#[serde_as]
#[derive(TypedBuilder, Serialize, Debug)]
pub struct Params {
    #[serde_as(as = "DisplayFromStr")]
    pair: Pair,
    order_ids: Vec<u64>,
}

#[derive(Deserialize)]
struct Response {
    orders: Vec<OrderInfo>,
}

pub async fn get(cred: Credential, params: Params) -> anyhow::Result<Vec<OrderInfo>> {
    let json = serde_json::to_string(&params)?;
    let resp: Response = ApiExec { cred }
        .post("/v1/user/spot/orders_info", json)
        .await?;
    Ok(resp.orders)
}
