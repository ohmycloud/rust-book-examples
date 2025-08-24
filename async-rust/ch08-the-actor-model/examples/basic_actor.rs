use std::sync::Arc;

use tokio::sync::mpsc::{Receiver, channel};
use tokio::sync::{Mutex, oneshot};

struct Message {
    value: i64,
}

struct RespMessage {
    value: i64,
    responder: oneshot::Sender<i64>,
}

async fn basic_actor(mut rx: Receiver<Message>) {
    let mut state = 0;

    while let Some(msg) = rx.recv().await {
        state += msg.value;
        println!("Received: {}", msg.value);
        println!("State: {}", state);
    }
}

async fn resp_actor(mut rx: Receiver<RespMessage>) {
    let mut state = 0;

    while let Some(msg) = rx.recv().await {
        state += msg.value;
        if msg.responder.send(state).is_err() {
            eprint!("Failed to send response");
        }
    }
}

async fn send_message() {
    let (tx, rx) = channel::<Message>(100);

    let _actor_handle = tokio::spawn(basic_actor(rx));
    for i in 0..10 {
        let msg = Message { value: i };
        tx.send(msg).await.unwrap();
    }
}

async fn oneshot_message() {
    let (tx, rx) = channel::<RespMessage>(100);

    let _resp_actor_handle = tokio::spawn(async {
        resp_actor(rx).await;
    });

    for i in 0..10 {
        let (resp_tx, resp_rx) = oneshot::channel::<i64>();
        let msg = RespMessage {
            value: i,
            responder: resp_tx,
        };
        tx.send(msg).await.unwrap();
        println!("Respoinse: {}", resp_rx.await.unwrap());
    }
}

async fn actor_replacement(state: Arc<Mutex<i64>>, value: i64) -> i64 {
    let mut state = state.lock().await;
    *state += value;
    *state
}

async fn actors_vs_mutexes() {
    let state = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    let now = tokio::time::Instant::now();

    for i in 0..100_000_000 {
        let state_ref = state.clone();
        let future = async move {
            let handle = tokio::spawn(async move { actor_replacement(state_ref, i).await });
            let _ = handle.await.unwrap();
        };
        handles.push(tokio::spawn(future));
    }

    for handle in handles {
        let _ = handle.await.unwrap();
    }
    println!("Elapsed: {:?}", now.elapsed());
}

#[tokio::main]
async fn main() {
    send_message().await;
    oneshot_message().await;
    actors_vs_mutexes().await;
}
