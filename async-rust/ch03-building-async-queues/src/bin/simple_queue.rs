use async_task::{Runnable, Task};
use futures_lite::future;
use std::pin::Pin;
use std::sync::LazyLock;
use std::task::{Context, Poll};
use std::time::Duration;
use std::time::Instant;
use std::{future::Future, panic::catch_unwind, thread};

// We pass a future into the function
// The function then converts the future into a task
// and puts the task on the queue to be executed.
fn spawn_task<F, T>(future: F) -> Task<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    static QUEUE: LazyLock<flume::Sender<Runnable>> = LazyLock::new(|| {
        // Create a channel
        let (tx, rx) = flume::unbounded::<Runnable>();

        // Have a thread waiting for tasks to be send to that thread
        // to be processed
        thread::spawn(move || {
            // check if reveived a Runnable
            while let Ok(runnable) = rx.recv() {
                println!("runnable accepted");
                // catches any error
                let _ = catch_unwind(|| runnable.run());
            }
        });
        // return the transmitter channel
        // so we can send runnable to our thread
        tx
    });

    // Create a closure that accepts a runnable and sends it to our queue.
    let schedule = |runnable| QUEUE.send(runnable).unwrap();
    // create the runnable and task by using `async_task`
    let (runable, task) = async_task::spawn(future, schedule);

    // When we chhedule the runnable,
    // we essentially put the task on the queue to be processed.
    // If we did not schedule the runnable, the task would not be run,
    // and our program would crash when we try to block tte main thread to
    // wait on the task being executed.
    runable.schedule();
    println!("Here is the queue count: {:?}", QUEUE.len());
    return task;
}

struct CounterFuture {
    count: u32,
}

impl Future for CounterFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        println!("polling with result: {}", self.count);
        std::thread::sleep(Duration::from_secs(1));
        if self.count < 3 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}

async fn async_fn() {
    std::thread::sleep(Duration::from_secs(1));
    println!("async fn");
}

struct AsyncSleep {
    start_time: Instant,
    duration: Duration,
}

impl AsyncSleep {
    fn new(duration: Duration) -> Self {
        Self {
            start_time: Instant::now(),
            duration,
        }
    }
}

impl Future for AsyncSleep {
    type Output = bool;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let elapsed_time = self.start_time.elapsed();
        // check the time elapsed between now and `start_time` on every poll
        if elapsed_time >= self.duration {
            Poll::Ready(true)
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn main() {
    let future_one = CounterFuture { count: 0 };
    let future_two = CounterFuture { count: 0 };
    let future_three = AsyncSleep::new(Duration::from_secs(5));

    let task_one = spawn_task(future_one);
    let task_two = spawn_task(future_two);
    let task_three = spawn_task(future_three);
    let task_four = spawn_task(async {
        async_fn().await;
        async_fn().await;
        async_fn().await;
        async_fn().await;
    });

    std::thread::sleep(Duration::from_secs(5));
    println!("before the block");
    future::block_on(task_one);
    future::block_on(task_two);
    future::block_on(task_three);
    future::block_on(task_four)
}
