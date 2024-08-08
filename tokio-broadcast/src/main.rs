use tokio::sync::broadcast;

// I used this channel to send updates from the bot back to the server to forward
// it to the client. The broadcast channel supports sending many values from many
// producers to many consumers. Each consumer will receive each value.


#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    tx.send("Message before subscribe").unwrap();

    // bot update
    tokio::spawn(async move {
        while let Ok(message) = rx1.recv().await {
            println!("Receiver1 got: {}", message);
        }
    });

    // Server receiving updates
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        while let Ok(message) = rx2.recv().await {
            println!("Receiver2 got: {}", message);
        }
    });

    tx.send("Message after subscribe").unwrap();
}

// Output:
// Receiver1 got: Message before subscribe
// Receiver1 got: Message after subscribe
// Receiver2 got: Message after subscribe
