use std::sync::Arc;

use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

// I used an mpsc channel to build a trading engine server, forwarding commands from
// clients to a server and then to a trading bot. The mpsc channel supports multiple
// producers sending values to a single consumer.

struct AppState;
struct Runner;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let state = Arc::new(AppState);
    let bot_runner = Arc::new(Runner);

    loop {
        // Assume Axum router is initialized
        // ws.on_upgrade(|socket| async move {
        //     handle_socket(socket, state.clone(), bot_runner.clone()).await;
        // });
        let (socket, _) = listener.accept().await.unwrap();
        let state = state.clone();
        let bot_runner = bot_runner.clone();
        tokio::spawn(async move {
            handle_socket(socket, state, bot_runner).await;
        });
    }
}

async fn handle_socket(mut socket: TcpStream, _app_state: Arc<AppState>, _bot_runner: Arc<Runner>) {
    let (internal_tx, mut internal_rx) = mpsc::channel::<String>(100);
    let mut tx_task = tokio::spawn(async move {
        while let Some(command) = internal_rx.recv().await {
            // Process command
            println!("Received command: {}", command);
            // Forwards the command to the bot that runs in a separate thread/task
        }
    });

    let mut rx_task = tokio::spawn(async move {
        let mut buf = [0; 1024];
        while let Ok(n) = socket.read(&mut buf).await {
            // Receive command from the client web socket and send it to the bot using the internal_tx
            if n == 0 {
                break;
            }
            let command = String::from_utf8_lossy(&buf[..n]).to_string();
            internal_tx.send(command).await.unwrap();
        }
    });

    tokio::select! {
        _ = (&mut  tx_task) => {
            rx_task.abort();
        }
        _ = (&mut rx_task) => {
            tx_task.abort();
        }
    }
}