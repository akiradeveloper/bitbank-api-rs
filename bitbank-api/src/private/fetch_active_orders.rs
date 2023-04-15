use super::*;

pub use super::fetch_order::OrderInfo;

#[derive(Deserialize)]
struct Response {
    orders: Vec<OrderInfo>,
}

#[derive(TypedBuilder, QueryParams, Debug)]
pub struct Params {
    pair: Pair,
    count: Option<u64>,
    from_id: Option<u64>,
    end_id: Option<u64>,
    since: Option<u64>,
    end: Option<u64>,
}

pub async fn get(cred: Credential, params: Params) -> anyhow::Result<Vec<OrderInfo>> {
    let resp: Response = ApiExec { cred }
        .get("/v1/user/spot/active_orders", params.to_query_params())
        .await?;
    Ok(resp.orders)
}
