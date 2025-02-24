use super::*;
use reqwest::Client;

/// Get asset list.
pub mod assets;
mod auth;
/// Cancel an order.
pub mod cancel_order;
/// Cancel multiple orders.
pub mod cancel_orders;
/// Create an order.
pub mod create_order;
/// Get deposit history.
pub mod deposit_history;
/// Get active orders.
pub mod fetch_active_orders;
/// Get an order information.
pub mod fetch_order;
/// Get multiple orders.
pub mod fetch_orders;
/// Get trade history.
pub mod trade_history;
/// Get withdrawal history.
pub mod withdrawal_history;

pub use auth::Credential;

struct ApiExec {
    cred: auth::Credential,
}
impl ApiExec {
    /// path: /v1/x/y/z
    /// params: a=b&c=d
    async fn get<R: serde::de::DeserializeOwned>(
        self,
        path: impl Into<String>,
        params: String,
    ) -> anyhow::Result<R> {
        let path = path.into();
        let params = if params.len() > 0 {
            format!("?{params}")
        } else {
            format!("")
        };

        let auth_headers = auth::GetAuth {
            path: path.clone(),
            params: params.clone(),
        }
        .create(self.cred)?;

        let url = format!("https://api.bitbank.cc{path}{params}");
        let (cli, req) = Client::new().get(url).headers(auth_headers).build_split();
        let resp: Response = cli.execute(req?).await?.json().await?;
        let data = resp.result()?;
        let data: R = serde_json::from_value(data)?;
        Ok(data)
    }

    /// path: /v1/x/y/z
    /// body: json
    async fn post<R: serde::de::DeserializeOwned>(
        self,
        path: impl Into<String>,
        body: String,
    ) -> anyhow::Result<R> {
        let path = path.into();
        let auth_headers = auth::PostAuth { body: body.clone() }.create(self.cred)?;
        let url = format!("https://api.bitbank.cc{path}");
        let (cli, req) = Client::new()
            .post(url)
            .headers(auth_headers)
            .body(body)
            .build_split();
        let resp: Response = cli.execute(req?).await?.json().await?;
        let data = resp.result()?;
        let data: R = serde_json::from_value(data)?;
        Ok(data)
    }
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct OrderInfo {
    pub order_id: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub pair: Pair,
    #[serde_as(as = "DisplayFromStr")]
    pub side: Side,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde_as(as = "DisplayFromStr")]
    pub start_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub remaining_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub executed_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    /// post_only only exists iff the order-type is limit otherwise omitted.
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub post_only: Option<bool>,
    #[serde_as(as = "DisplayFromStr")]
    pub average_price: f64,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub ordered_at: NaiveDateTime,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub expire_at: Option<NaiveDateTime>,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub triggered_at: Option<NaiveDateTime>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub trigger_price: Option<f64>,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub canceled_at: Option<NaiveDateTime>,
    #[serde_as(as = "DisplayFromStr")]
    pub status: OrderStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_parse() -> anyhow::Result<()> {
        let url = format!("https://api.bitbank.cc/v1/some_api?a=b&c=d");
        let cli = Client::new();
        let req = cli.get(url).build()?;
        dbg!(&req);
        Ok(())
    }
}
