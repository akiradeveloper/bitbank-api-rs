use super::*;

#[derive(Debug, serde::Deserialize)]
struct Response {
    candlestick: Vec<CandlestickResponse>,
}

#[derive(Debug, serde::Deserialize)]
struct CandlestickResponse {
    ohlcv: Vec<serde_json::Value>,
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
    fn from(mut value: serde_json::Value) -> Option<Self> {
        let arr = value.as_array_mut().unwrap();
        arr.reverse();
        let mut out = [0.; 5];
        for i in 0..5 {
            let v: f64 = arr.pop()?.as_str()?.parse().ok()?;
            out[i] = v;
        }
        let timestamp = {
            let ms = arr.pop()?.as_i64()?;
            NaiveDateTime::from_timestamp_millis(ms)?
        };
        Some(Self {
            start: out[0],
            high: out[1],
            low: out[2],
            end: out[3],
            volume: out[4],
            timestamp,
        })
    }
}

#[derive(Debug, Clone, derive_more::Display, serde::Deserialize)]
pub enum CandleType {
    #[display(fmt = "1min")]
    Min1,
    #[display(fmt = "5min")]
    Min5,
    #[display(fmt = "15min")]
    Min15,
    #[display(fmt = "30min")]
    Min30,
    #[display(fmt = "1hour")]
    Hour1,

    #[display(fmt = "4hour")]
    Hour4,
    #[display(fmt = "8hour")]
    Hour8,
    #[display(fmt = "12hour")]
    Hour12,
    #[display(fmt = "1day")]
    Day1,
    #[display(fmt = "1week")]
    Week1,
    #[display(fmt = "1month")]
    Month1,
}

#[derive(Clone, derive_more::Display)]
pub enum Period {
    #[display(fmt = "{_0}")]
    YYYY(u16),
    #[display(fmt = "{y}{m}{d}")]
    YYYYMMDD { y: u16, m: u8, d: u8 },
}

#[derive(Builder)]
#[builder(setter(into))]
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
    let mut resp: Response = do_get(path(params)).await?;
    let resp = resp
        .candlestick
        .pop()
        .ok_or(anyhow::anyhow!("response is empty"))?;
    let mut out = vec![];
    for x in resp.ohlcv {
        let y = Candlestick::from(x).ok_or(anyhow::anyhow!("failed to parse a candlestick"))?;
        out.push(y);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_path() -> anyhow::Result<()> {
        let params = ParamsBuilder::default()
            .pair(Pair::xrp_jpy)
            .candle_type(CandleType::Day1)
            .period(Period::YYYY(2023))
            .build()?;
        let path = path(params);
        assert_eq!(path, "/xrp_jpy/candlestick/1day/2023");
        Ok(())
    }

    #[tokio::test]
    async fn test_candlestick_yyyy() -> anyhow::Result<()> {
        let params = ParamsBuilder::default()
            .pair(Pair::xrp_jpy)
            .candle_type(CandleType::Month1)
            .period(Period::YYYY(2022))
            .build()?;
        let resp = get(params).await?;
        dbg!(&resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_candlestick_yyyymmdd() -> anyhow::Result<()> {
        let params = ParamsBuilder::default()
            .pair(Pair::xrp_jpy)
            .candle_type(CandleType::Min15)
            .period(Period::YYYYMMDD {
                y: 2022,
                m: 12,
                d: 25,
            })
            .build()?;
        let resp = get(params).await?;
        dbg!(&resp);
        Ok(())
    }
}
