use super::*;
use reqwest::Client;

use derive_builder::Builder;
use query_params::QueryParams;

mod auth;
pub mod cancel_order;
pub mod create_order;
pub mod fetch_order;
pub mod trade_history;

pub use auth::Credential;

struct ApiExec {
    cred: auth::Credential,
}
impl ApiExec {
    /// path: /v1/x/y/z
    /// params: ?a=b&c=d
    async fn get<R: serde::de::DeserializeOwned>(
        self,
        path: impl Into<String>,
        params: String,
    ) -> anyhow::Result<R> {
        let path = path.into();
        let auth_headers = auth::GetAuth {
            path: path.clone(),
            params: params.clone(),
        }
        .create(self.cred)?;
        let url = format!("https://api.bitbank.cc{path}{params}");
        let (cli, req) = Client::new().get(url).headers(auth_headers).build_split();
        dbg!(&req);
        let resp: Response = cli.execute(req?).await?.json().await?;
        anyhow::ensure!(resp.success == 1);
        let data: R = serde_json::from_value(resp.data)?;
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
            .json(&body)
            .build_split();
        dbg!(&req);
        let resp: Response = cli.execute(req?).await?.json().await?;
        anyhow::ensure!(resp.success == 1);
        let data: R = serde_json::from_value(resp.data)?;
        Ok(data)
    }
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