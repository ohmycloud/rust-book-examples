use std::{sync::Arc, time::Duration};
use tokio::task::JoinHandle;

#[derive(Debug)]
enum CounterType {
    Increment,
    Decrement,
}

struct SharedData {
    counter: i32,
}

impl SharedData {
    fn increment(&mut self) {
        self.counter += 1;
    }

    fn decrement(&mut self) {
        self.counter -= 1;
    }
}

async fn count(
    count: u32,
    data: Arc<tokio::sync::Mutex<SharedData>>,
    counter_type: CounterType,
) -> u32 {
    for _ in 0..count {
        let mut data = data.lock().await;
        match counter_type {
            CounterType::Increment => {
                data.increment();
                println!("after increment: {}", data.counter);
            }
            CounterType::Decrement => {
                data.decrement();
                println!("after decrement: {}", data.counter);
            }
        }
        std::mem::drop(data);
        std::thread::sleep(Duration::from_secs(1));
    }
    return count;
}

#[tokio::main]
async fn main() {
    let shared_data = Arc::new(tokio::sync::Mutex::new(SharedData { counter: 0 }));
    let shared_two = shared_data.clone();

    let increment_counter: JoinHandle<u32> =
        tokio::task::spawn(async move { count(3, shared_data, CounterType::Increment).await });
    let decrement_counter: JoinHandle<u32> =
        tokio::task::spawn(async move { count(3, shared_two, CounterType::Decrement).await });
    tokio::join!(increment_counter, decrement_counter);
}
