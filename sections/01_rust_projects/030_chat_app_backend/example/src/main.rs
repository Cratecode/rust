use axum::extract::{
    ws::{Message, WebSocket},
    State, WebSocketUpgrade,
};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::Router;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct ServerState {
    /// The list of every message that this server has received.
    message_history: Arc<Mutex<Vec<String>>>,
    /// A channel to share messages between all connected clients.
    message_channel: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    let message_history = Arc::new(Mutex::new(Vec::new()));
    let (tx, _rx) = broadcast::channel(32);

    let state = ServerState {
        // This is syntax sugar for message_history: message_history.
        message_history,
        message_channel: tx,
    };

    let app = Router::new()
        .route("/socket", any(ws_handler))
        .fallback_service(ServeDir::new("public"))
        .with_state(state);

    let listener = TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<ServerState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: ServerState) {
    // Send over every message to the client.
    let messages = state.message_history.lock().unwrap().clone();

    for msg in messages {
        if let Err(err) = socket.send(Message::text(msg)).await {
            eprintln!("Error while sending initial data: {err:?}");
            return;
        }
    }

    let mut recv = state.message_channel.subscribe();

    // Receive loop.
    loop {
        tokio::select! {
            val = recv.recv() => {
                let Ok(val) = val else {
                    break;
                };

                if let Err(err) = socket.send(Message::text(val)).await {
                    eprintln!("Error while sending message to socket: {err:?}");
                    return;
                }
            }
            val = socket.recv() => {
                let Some(Ok(val)) = val else {
                    break;
                };

                // Only handle text messages.
                if let Ok(text) = val.into_text() {
                    // Send message to all connected clients.
                    if state.message_channel.send(text.to_string()).is_err() {
                        break;
                    }

                    // Save message to history.
                    state.message_history.lock().unwrap().push(text.to_string());
                }
            }
        }
    }
}
