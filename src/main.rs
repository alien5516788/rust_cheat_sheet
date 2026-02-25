pub mod advanced;
pub mod basics;
pub mod flow_control;
pub mod miscellaneous;

use tokio::time::{sleep, Duration};

async fn task(name: &str) {
    println!("{} started", name);
    sleep(Duration::from_secs(5)).await;
    println!("{} finished", name);
}

#[tokio::main]
async fn main() {
    let t1 = task("A");
    let t2 = task("B");
    
    tokio::spawn(t1);
    tokio::spawn(t2);
    
    sleep(Duration::from_secs(5)).await;
    println!("Main function");
    // tokio::join!(t1, t2);
}
