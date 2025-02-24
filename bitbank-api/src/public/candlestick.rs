use super::*;

mod raw {
    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        pub candlestick: Candlestick,
    }
    #[derive(Deserialize, Debug)]
    pub struct Candlestick(
        pub Ohlcv,
        // This is a workaround.
        // Without this, newtype tries to deserialize as x not as [x]
        #[serde(skip)] (),
    );

    #[derive(Deserialize, Debug)]
    pub struct Ohlcv {
        pub ohlcv: Vec<OhlcvItem>,
    }

    #[serde_as]
    #[derive(Deserialize, Debug)]
    pub struct OhlcvItem(
        #[serde_as(as = "DisplayFromStr")] pub f64,
        #[serde_as(as = "DisplayFromStr")] pub f64,
        #[serde_as(as = "DisplayFromStr")] pub f64,
        #[serde_as(as = "DisplayFromStr")] pub f64,
        #[serde_as(as = "DisplayFromStr")] pub f64,
        #[serde_as(as = "TimestampMilliSeconds")] pub NaiveDateTime,
    );
}

#[derive(Debug)]
pub struct Candlestick {
    pub start: f64,
    pub high: f64,
    pub low: f64,
    pub end: f64,
    pub volume: f64,
    pub timestamp: NaiveDateTime,
}
impl Candlestick {
    fn new(x: raw::OhlcvItem) -> Self {
        Candlestick {
            start: x.0,
            high: x.1,
            low: x.2,
            end: x.3,
            volume: x.4,
            timestamp: x.5,
        }
    }
}

#[derive(derive_more::Display, Debug, Clone)]
pub enum CandleType {
    #[display("1min")]
    Min1,
    #[display("5min")]
    Min5,
    #[display("15min")]
    Min15,
    #[display("30min")]
    Min30,
    #[display("1hour")]
    Hour1,

    #[display("4hour")]
    Hour4,
    #[display("8hour")]
    Hour8,
    #[display("12hour")]
    Hour12,
    #[display("1day")]
    Day1,
    #[display("1week")]
    Week1,
    #[display("1month")]
    Month1,
}

#[derive(derive_more::Display, Clone)]
pub enum Period {
    #[display("{_0}")]
    YYYY(u16),
    #[display("{_0}{_1}{_2}")]
    YYYYMMDD(u16, u8, u8),
}

#[derive(TypedBuilder)]
pub struct Params {
    pair: Pair,
    candle_type: CandleType,
    period: Period,
}

fn path(params: Params) -> String {
    format!(
        "/{}/candlestick/{}/{}",
        params.pair, params.candle_type, params.period
    )
}

pub async fn get(params: Params) -> anyhow::Result<Vec<Candlestick>> {
    let resp: raw::Response = do_get(path(params)).await?;
    let ohlcv = resp.candlestick.0;
    Ok(ohlcv.ohlcv.into_iter().map(Candlestick::new).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_path() -> anyhow::Result<()> {
        let params = Params::builder()
            .pair(Pair(XRP, JPY))
            .candle_type(CandleType::Day1)
            .period(Period::YYYY(2023))
            .build();
        let path = path(params);
        assert_eq!(path, "/xrp_jpy/candlestick/1day/2023");
        Ok(())
    }

    #[tokio::test]
    async fn test_candlestick_yyyy() -> anyhow::Result<()> {
        let params = Params::builder()
            .pair(Pair(XRP, JPY))
            .candle_type(CandleType::Month1)
            .period(Period::YYYY(2022))
            .build();
        let resp = get(params).await?;
        dbg!(&resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_candlestick_yyyymmdd() -> anyhow::Result<()> {
        let params = Params::builder()
            .pair(Pair(XRP, JPY))
            .candle_type(CandleType::Min15)
            .period(Period::YYYYMMDD(2022, 12, 25))
            .build();
        let resp = get(params).await?;
        dbg!(&resp);
        Ok(())
    }
}
