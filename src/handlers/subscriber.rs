use crate::message::Message;
use crate::state::State;
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};
use std::sync::Arc;

pub async fn subscriber_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<State>) {
    let mut subscribers = state.subscribers.lock().await;
    println!("{:?}", subscribers.keys());
    if let Some(Ok(msg)) = socket.recv().await {
        let msg = Message::from(msg);
        if let Message::Text(msg) = msg {
            println!("{}", msg);
            if let Ok(id) = uuid::Uuid::parse_str(&msg) {
                if let Some(subscribers) = subscribers.get_mut(&id) {
                    subscribers.push(socket);
                }
            }
        }
    };
}
