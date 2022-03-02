use std::sync::Arc;
use tokio::sync::Barrier;

#[tokio::main]
async fn main() {
    let barrier = Arc::new(Barrier::new(3));

    let b2 = barrier.clone();
    tokio::spawn(async move {
        // do something
        b2.wait().await;
    });

    let b3 = barrier.clone();
    tokio::spawn(async move {
        // do something
        b3.wait().await;
    });

    barrier.wait().await;

    barrier.wait().await;

    println!("This will print only when all the three concurrent operations ahve terminated");
}
