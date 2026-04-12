//! AlphaIru
//! Twitch Reader
//! 
//! nico.rs
//!
//! This is the module that handles the
//! connection to Nico HTML/JS (WebSocket).
//!

use std::fs::{create_dir_all, write};
use std::env;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
    Router
};
use tokio::sync::broadcast;
use crate::ChatPayload;

pub async fn start_nico_server(
    tx: broadcast::Sender<ChatPayload>,
) {

    let verbose: bool = env::var("VERBOSE_LOG").unwrap_or_else(|_| "false".to_string()) == "true";
    let address: String = env::var("NICO_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:2578".to_string());

    let _ = tx.send(ChatPayload {
        username: "[SYSTEM]".to_string(),
        user_id: "0".to_string(),
        msg: "Nico (WebSocket) is active.".to_string(),
        color: "#FFFF66".to_string(),
        ..Default::default()
    });

    if verbose {
        let _ = tx.send(ChatPayload {
            username: "[SYSTEM]".to_string(),
            user_id: "0".to_string(),
            msg: format!("Listening on: {}", address),
            color: "#FFFF66".to_string(),
            ..Default::default()
        });
    }
    
    let js_config = format!("const NICO_ADDR = '{}';", address);
    create_dir_all("web").ok();
    write("web/config.js", js_config).unwrap();

    let tx_for_router = tx.clone();

    let app = Router::new().route("/ws", get(move |ws: WebSocketUpgrade| { 
        let tx_for_ws = tx_for_router.clone();
        async move {
            ws.on_upgrade(move |socket| handle_socket(socket, tx_for_ws.subscribe())) 
        }
    }));

    let listener = match tokio::net::TcpListener::bind(&address).await {
    Ok(l) => l,
    Err(e) => {
        let _ = tx.send(ChatPayload {
            username: "[ERROR]".to_string(),
            msg: format!("Failed to start WebSocket: {}", e),
            color: "#FF0000".to_string(),
            ..Default::default()
        });
        return;
    }
};

    axum::serve(listener, app).await.unwrap();


}

async fn handle_socket(
    mut socket: WebSocket,
    mut rx: broadcast::Receiver<ChatPayload>
) {
    while let Ok(payload) = rx.recv().await {
        let msg = payload.msg;
        if socket.send(Message::Text(msg.into())).await.is_err() {
            break;            
        };
    }
}
