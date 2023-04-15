use super::*;

pub mod candlestick;
pub mod depth;
pub mod exchange_status;
pub mod pairs;
pub mod ticker;
pub mod tickers;
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
