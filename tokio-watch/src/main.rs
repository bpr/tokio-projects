use tokio::sync::watch;

// watch supports sending many values from many producers to many consumers, but only
// the most recent value is stored. Consumers are notified when a new value is sent.

#[tokio::main]
async fn main() {
    let (tx, rx) = watch::channel("init");

    // Value update
    tokio::spawn(async move {
        tx.send("New value").unwrap();
    });

    // Value consumer
    let mut rx_clone = rx.clone();
    tokio::spawn(async move {
        while rx_clone.changed().await.is_ok() {
            println!("Received: {}", *rx_clone.borrow());
        }
    });

}
