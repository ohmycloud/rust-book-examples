use std::{
    pin::Pin,
    task::{Context, Poll},
};

trait Greeting {
    fn greet(&self) -> String;
}

struct HelloWorld;

impl Greeting for HelloWorld {
    fn greet(&self) -> String {
        "Hello, World!".to_string()
    }
}

impl Greeting for &HelloWorld {
    fn greet(&self) -> String {
        "Hello, World!".to_string()
    }
}

struct ExcitedGreeting<T> {
    inner: T,
}

impl<T> ExcitedGreeting<T> {
    fn greet(&self) -> String
    where
        T: Greeting,
    {
        let mut greeting = self.inner.greet();
        greeting.push_str("I'm so excited to be in Rust!");
        greeting
    }
}

fn feature_flag_decorator() {
    let raw_one = HelloWorld;
    let raw_two = HelloWorld;

    #[cfg(feature = "logging_decorator")]
    let decorated = ExcitedGreeting { inner: raw_two };
    println!("{}", raw_one.greet());
    println!("{}", decorated.greet());
}

trait Logging {
    fn log(&self);
}

struct LoggingFuture<F: Future + Logging> {
    inner: F,
}

impl<F: Future + Logging> Future for LoggingFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = unsafe { self.map_unchecked_mut(|s| &mut s.inner) };
        inner.log();
        inner.poll(cx)
    }
}

impl<F: Future> Logging for F {
    fn log(&self) {
        println!("Polling the future!")
    }
}

async fn my_async_function() -> String {
    "Result of async computation".to_string()
}

async fn logging_future() -> String {
    let logged_future = LoggingFuture {
        inner: my_async_function(),
    };
    let result = logged_future.await;
    result
}

#[tokio::main]
async fn main() {
    feature_flag_decorator();
    let result = logging_future().await;
    println!("{}", result);
}
