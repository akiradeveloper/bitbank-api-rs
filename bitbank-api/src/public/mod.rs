use super::*;

/// Get candlestick information.
pub mod candlestick;
/// Get depth information.
pub mod depth;
/// Get exhange status.
pub mod exchange_status;
/// Get all pairs information.
pub mod pairs;
/// Get ticker information.
pub mod ticker;
/// Get all tickers information.
pub mod tickers;
/// Get executed transactions.
pub mod transactions;

async fn do_get<R: serde::de::DeserializeOwned>(path: String) -> anyhow::Result<R> {
    let url = format!("https://public.bitbank.cc{path}");
    let cli = reqwest::Client::new();
    let resp: Response = cli.get(url).send().await?.json().await?;
    let data = resp.result()?;
    let data: R = serde_json::from_value(data)?;
    Ok(data)
}

async fn do_get_private<R: serde::de::DeserializeOwned>(
    path: &str,
    params: String,
) -> anyhow::Result<R> {
    let url = format!("https://api.bitbank.cc{path}{params}");
    let cli = reqwest::Client::new();
    let resp: Response = cli.get(url).send().await?.json().await?;
    let data = resp.result()?;
    let data: R = serde_json::from_value(data)?;
    Ok(data)
}
