use super::*;
use std::{cell::RefCell, rc::Rc};
use tokio_tungstenite::tungstenite::Message;

pub mod ticker;
pub mod transactions;

enum ParsedMessage<R: serde::de::DeserializeOwned> {
    Parsed(R),
    Ping,
}

fn parse_message<R: serde::de::DeserializeOwned>(msg: Message) -> Option<ParsedMessage<R>> {
    use std::str::FromStr;

    let txt = msg.into_text().ok()?;

    // Find the boundary between number and JSON
    let boundary = txt
        .char_indices()
        .find(|(_, c)| !c.is_ascii_digit())
        .map(|(idx, _)| idx)
        .unwrap_or(txt.len());

    // If the index or txt.len() is 0, no number to parse
    if boundary == 0 {
        return None;
    }

    match u8::from_str(&txt[0..boundary]).ok()? {
        42 => {}
        2 => return Some(ParsedMessage::Ping),
        _ => return None,
    }

    let mut v = serde_json::from_str::<serde_json::Value>(&txt[boundary..]).ok()?;
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
    serde_json::from_value::<R>(data)
        .ok()
        .map(ParsedMessage::Parsed)
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

    let rc_writer = Rc::new(RefCell::new(writer));
    let st = reader.filter_map(move |msg| {
        let fut_writer = Rc::clone(&rc_writer);
        async move {
            let msg = msg.ok()?;

            use ParsedMessage::*;
            match parse_message::<R>(msg)? {
                Parsed(v) => Some(v),
                Ping => {
                    // Send Pong
                    fut_writer
                        .borrow_mut()
                        .send(Message::text("3"))
                        .await
                        .ok()?;
                    None
                }
            }
        }
    });
    Ok(st)
}
