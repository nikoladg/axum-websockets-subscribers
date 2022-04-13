use axum::extract::ws::WebSocket;
use futures::lock::Mutex;
use std::collections::HashMap;

pub struct State {
    pub destination_url: String,
    pub subscribers: Mutex<HashMap<uuid::Uuid, Vec<WebSocket>>>,
}
