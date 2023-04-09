use super::*;

pub async fn get(pair: Pair) -> anyhow::Result<Ticker> {
    let url = format!("https://public.bitbank.cc/{}/ticker", pair);
    do_get(url).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let x = get(Pair::xrp_jpy).await?;
        Ok(())
    }
}
