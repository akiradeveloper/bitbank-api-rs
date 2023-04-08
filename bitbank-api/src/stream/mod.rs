use super::*;
use tokio_tungstenite::tungstenite::Message;

pub mod ticker;

fn parse_message<R: serde::de::DeserializeOwned>(msg: Message) -> Option<R> {
    use std::str::FromStr;

    let txt = msg.into_text().ok()?;
    if txt.len() < 2 {
        return None;
    }
    // Assume the response is in form 42[...]
    let n = u8::from_str(&txt[0..2]).ok()?;
    if n != 42 {
        return None;
    }
    let mut v = serde_json::from_str::<serde_json::Value>(&txt[2..]).ok()?;
    let mut body = {
        let tmp = v.as_array_mut()?;
        tmp.remove(1)
    };
    let mut message = {
        let tmp = body.as_object_mut()?;
        tmp.remove("message")?
    };
    let data = {
        let tmp = message.as_object_mut()?;
        tmp.remove("data")?
    };
    serde_json::from_value::<R>(data).ok()
}

async fn do_connect<R: serde::de::DeserializeOwned>(
    room_id: &str,
) -> anyhow::Result<impl tokio_stream::Stream<Item = R>> {
    use futures_util::{SinkExt, StreamExt};

    let url = "wss://stream.bitbank.cc/socket.io/?EIO=4&transport=websocket";
    let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
    let (mut writer, reader) = ws_stream.split();

    // Request connection
    writer.send(Message::text("40")).await?;

    let msg = format!("42[\"join-room\", \"{room_id}\"]");
    writer.send(Message::Text(msg)).await?;

    let st = reader.filter_map(|msg| async move {
        match msg {
            Ok(msg) => parse_message::<R>(msg),
            _ => None,
        }
    });
    Ok(st)
}
