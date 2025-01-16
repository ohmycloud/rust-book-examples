use async_task::{Runnable, Task};
use flume::{Receiver, Sender};
use futures_lite::future;
use std::pin::Pin;
use std::sync::LazyLock;
use std::task::{Context, Poll};
use std::time::Duration;
use std::time::Instant;
use std::{future::Future, panic::catch_unwind, thread};

static HIGH_CHANNEL: LazyLock<(Sender<Runnable>, Receiver<Runnable>)> =
    LazyLock::new(|| flume::unbounded::<Runnable>());
static LOW_CHANNEL: LazyLock<(Sender<Runnable>, Receiver<Runnable>)> =
    LazyLock::new(|| flume::unbounded::<Runnable>());

#[derive(Debug, Clone, Copy)]
enum FutureType {
    High,
    Low,
}

// We pass a future into the function
// The function then converts the future into a task
// and puts the task on the queue to be executed.
fn spawn_task<F, T>(future: F, order: FutureType) -> Task<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    static HIGH_QUEUE: LazyLock<Sender<Runnable>> = LazyLock::new(|| {
        // Have multi threads waiting for tasks to be send to that thread
        // to be processed
        for _ in 0..2 {
            // Create channels
            let high_receiver = HIGH_CHANNEL.1.clone();
            let low_receiver = LOW_CHANNEL.1.clone();

            thread::spawn(move || {
                loop {
                    match high_receiver.try_recv() {
                        Ok(runnable) => {
                            let _ = catch_unwind(|| runnable.run());
                        }
                        Err(_) => match low_receiver.try_recv() {
                            Ok(runnable) => {
                                let _ = catch_unwind(|| runnable.run());
                            }
                            Err(_) => {
                                thread::sleep(Duration::from_millis(100));
                            }
                        },
                    }
                }
            });
        }

        // return the transmitter channel
        // so we can send runnable to our thread
        HIGH_CHANNEL.0.clone()
    });

    static LOW_QUEUE: LazyLock<Sender<Runnable>> = LazyLock::new(|| {
        // Have only one thread waiting for tasks to be send to that thread
        // to be processed
        for _ in 0..1 {
            // Create channels
            let high_receiver = HIGH_CHANNEL.1.clone();
            let low_receiver = LOW_CHANNEL.1.clone();

            thread::spawn(move || {
                loop {
                    match high_receiver.try_recv() {
                        Ok(runnable) => {
                            let _ = catch_unwind(|| runnable.run());
                        }
                        Err(_) => match low_receiver.try_recv() {
                            Ok(runnable) => {
                                let _ = catch_unwind(|| runnable.run());
                            }
                            Err(_) => {
                                thread::sleep(Duration::from_millis(100));
                            }
                        },
                    }
                }
            });
        }

        // return the transmitter channel
        // so we can send runnable to our thread
        LOW_CHANNEL.0.clone()
    });

    let schedule = match order {
        FutureType::High => |runnable| HIGH_QUEUE.send(runnable).unwrap(),
        FutureType::Low => |runnable| LOW_QUEUE.send(runnable).unwrap(),
    };

    // create the runnable and task by using `async_task`
    let (runable, task) = async_task::spawn(future, schedule);

    // When we chhedule the runnable,
    // we essentially put the task on the queue to be processed.
    // If we did not schedule the runnable, the task would not be run,
    // and our program would crash when we try to block tte main thread to
    // wait on the task being executed.
    runable.schedule();
    return task;
}

macro_rules! spawn_task {
    ($future:expr) => {
        spawn_task!($future, FutureType::Low)
    };
    ($future:expr, $order:expr) => {
        spawn_task($future, $order)
    };
}

macro_rules! join {
    ($($future:expr),*) => {
        {
            let mut results = Vec::new();
            $(
                results.push(future::block_on($future));
            )*
            results
        }
    };
}

macro_rules! try_join {
    ($($future:expr),*) => {
        {
            let mut results = Vec::new();
            $(
                let result = catch_unwind(|| future::block_on($future));
                results.push(result);
            )*
            results
        }
    };
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

    let task_one = spawn_task!(future_one, FutureType::High);
    let task_two = spawn_task!(future_two);
    let task_three = spawn_task!(future_three);
    let task_four = spawn_task!(
        async {
            async_fn().await;
            async_fn().await;
            async_fn().await;
            async_fn().await;
        },
        FutureType::High
    );
    let task_five = spawn_task!(async_fn());

    std::thread::sleep(Duration::from_secs(5));
    println!("before the block");

    let outcome_one: Vec<u32> = join!(task_one, task_two);
    println!("{:?}", outcome_one);
    future::block_on(task_three);

    let outcome_two: Vec<Result<(), _>> = try_join!(task_four, task_five);
    println!("{:?}", outcome_two);
}
