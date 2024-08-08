use tokio::sync::oneshot;

// It supports sending a single value from a single producer to a single consumer.
// This is useful for sending the result of a one time computation to a waiter.

async fn some_computation() -> String {
    "Some computation".to_string()
}

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let res = some_computation().await;
        tx.send(res).unwrap();
    });

    let res = rx.await.unwrap();
    println!("Received: {}", res);
}
