use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use std::sync::Arc;

const MAX_CONCURRENT_CONNECTIONS: usize = 1000;

#[tokio::main]
async fn main() {
    println!("Starting...");

    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_CONNECTIONS));
    let mut tasks = Vec::new();

    for i in 1..254 {
        let ip = "127.0.0.";
        for x in 1..65536 {
            let f = format!("{}{}:{}", ip, i, x);
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            let task = tokio::spawn(async move {
                match TcpStream::connect(&f).await {
                    Ok(_) => {
                        println!("Connected to {}", f);
                    },
                    Err(_e) => {
                    }
                }
                drop(permit); 
            });

            tasks.push(task);
        }
    }

    for task in tasks {
        let _ = task.await;
    }

    println!("Done");
}
