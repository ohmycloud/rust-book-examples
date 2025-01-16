use async_task::{Runnable, Task};
use bytes::Bytes;
use flume::{Receiver, Sender};
use futures_lite::future;
use http::Uri;
use http_body_util::{Empty, Full};
use hyper::Request;
use hyper_util::client::legacy::Client;
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
        let high_num = std::env::var("HIGH_NUM").unwrap().parse::<usize>().unwrap();
        for _ in 0..high_num {
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
        // Have threads waiting for tasks to be send to that thread
        // to be processed
        let low_num = std::env::var("LOW_NUM").unwrap().parse::<usize>().unwrap();
        for _ in 0..low_num {
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

#[derive(Debug, Clone)]
struct Runtime {
    // the consuming threads for the high-priority queue
    high_num: usize,
    // the consuming threads for the low-priority queue
    low_num: usize,
}

impl Runtime {
    pub fn new() -> Self {
        let num_cores = std::thread::available_parallelism().unwrap().get();

        Self {
            high_num: num_cores - 2,
            low_num: 1,
        }
    }

    pub fn with_high_num(mut self, num: usize) -> Self {
        self.high_num = num;
        self
    }
    pub fn with_low_num(mut self, num: usize) -> Self {
        self.low_num = num;
        self
    }
    pub fn run(&self) {
        unsafe {
            std::env::set_var("HIGH_NUM", self.high_num.to_string());
            std::env::set_var("LOW_NUM", self.low_num.to_string());
        }

        let high = spawn_task!(async {}, FutureType::High);
        let low = spawn_task!(async {}, FutureType::Low);
        join!(high, low);
    }
}

#[derive(Debug, Clone, Copy)]
struct BackgroundProcess;

impl Future for BackgroundProcess {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("background process firing");
        std::thread::sleep(Duration::from_secs(1));
        cx.waker().wake_by_ref();
        Poll::Pending
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

    Runtime::new().with_high_num(4).with_low_num(2).run();
    // 在 Rust 的主函数中，如果我们直接丢弃(drop)一个任务，
    // 这个任务即使正在异步运行时中执行，也会被取消并停止执行
    let _background = spawn_task!(BackgroundProcess {}).detach();

    // 为了观察到后台进程的执行, 我们保持主线程运行一段时间,
    // 以防止后台任务被提前取消而停止执行
    println!("Main thread sleeping...");
    std::thread::sleep(Duration::from_secs(10));
    println!("Main thread exiting...");

    let url = "http://www.rust-lang.org";
    let uri: Uri = url.parse().unwrap();

    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .header("User-Agent", "hyper/1.5.2")
        .header("Accept", "text/html")
        .body(Empty::<Bytes>::new())
        .unwrap();
}
