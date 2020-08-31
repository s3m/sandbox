use futures::stream::FuturesUnordered;
use std::time::{Duration, Instant};
use tokio::stream::StreamExt;
use tokio::time;

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let mut tasks = FuturesUnordered::new();

    for t in 1..=100 {
        println!("push task, current tasks: {}", tasks.len());
        tasks.push(sleep(t));

        if tasks.len() == 4 {
            if let Some(t) = tasks.next().await {
                println!("{}", t);
            }
        }
    }

    while let Some(t) = tasks.next().await {
        println!("rs: {:#?}", t);
    }

    println!("Elapsed: {:.2?}", now.elapsed());
}

async fn sleep(i: usize) -> String {
    let zzz = if i % 3 == 0 { 1 } else { 5 };
    let _ = time::delay_for(Duration::from_secs(zzz)).await;
    format!("{} slept {} seconds", i, zzz)
}
