use super::*;

pub mod candlestick;
pub mod ticker;

async fn do_get<R: serde::de::DeserializeOwned>(path: String) -> anyhow::Result<R> {
    let url = format!("https://public.bitbank.cc{path}");
    let cli = reqwest::Client::new();
    let resp: Response = cli.get(url).send().await?.json().await?;
    let data = resp.result()?;
    let data: R = serde_json::from_value(data)?;
    Ok(data)
}
