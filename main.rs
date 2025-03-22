use std::time::Duration;
use tokio::time::sleep;

async fn task_one() {
    sleep(Duration::from_secs(2)).await;
    println!("Task One Done!");
}

async fn task_two() {
    sleep(Duration::from_secs(1)).await;
    println!("Task Two Done!");
}

#[tokio::main]
async fn main() {
    println!("Starting tasks...");

    // Run both tasks concurrently
    let (res1, res2) = tokio::join!(task_one(), task_two());

    println!("Both tasks finished!");
}
