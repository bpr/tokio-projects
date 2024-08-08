use tokio::sync::broadcast;

// I used this channel to send updates from the bot back to the server to forward
// it to the client. The broadcast channel supports sending many values from many
// producers to many consumers. Each consumer will receive each value.


#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(100);
    let tx_clone = tx.clone();

    // bot update
    tokio::spawn(async move {
        loop {
            let update = "bot update".to_string();
            tx_clone.send(update).unwrap();
            // await on some condition
        }
    });

    // Server receiving updates
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
        while let Ok(update) = rx.recv().await {
            println!("Received: {}", update);
        }
    });
}
