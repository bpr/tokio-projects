use tokio::sync::broadcast;

// I used this channel to send updates from the bot back to the server to forward
// it to the client. The broadcast channel supports sending many values from many
// producers to many consumers. Each consumer will receive each value.


#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(1);
    let mut rx1 = tx.subscribe();

    let task_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = rx1.recv() => {
                    println!("Shutdown signal received");
                    break;
                }
                _ = rx.recv() => {}
            }
        }
    });

    // Send shutdown signal

    tx.send(()).unwrap();
    task_handle.await.unwrap();
}

// Output:
// Shutdown signal received
