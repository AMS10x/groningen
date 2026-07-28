use std::time::Duration;

use crossterm::event::{Event, EventStream};
use futures_util::StreamExt;
use tokio::sync::mpsc;

use crate::app::AppEvent;

pub async fn listen(tx: mpsc::Sender<AppEvent>) {
    let mut reader = EventStream::new();
    let mut ticker = tokio::time::interval(Duration::from_millis(250));
    loop {
        tokio::select! {
            _ = ticker.tick() => {
                if tx.send(AppEvent::Tick).await.is_err() { break; }
            }
            maybe_event = reader.next() => {
                match maybe_event {
                    Some(Ok(Event::Key(key))) => {
                        if tx.send(AppEvent::Input(key)).await.is_err() { break; }
                    }
                    Some(Ok(_)) => {}
                    Some(Err(err)) => {
                        if tx.send(AppEvent::Error(err.to_string())).await.is_err() { break; }
                    }
                    None => break,
                }
            }
        }
    }
}
