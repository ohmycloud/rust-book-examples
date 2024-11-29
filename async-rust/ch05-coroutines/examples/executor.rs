#![feature(coroutines, coroutine_trait)]
use std::{
    collections::VecDeque,
    ops::{Coroutine, CoroutineState},
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};

struct Executor {
    corotinues: VecDeque<Pin<Box<dyn Coroutine<(), Yield = (), Return = ()>>>>,
}

impl Executor {
    fn new() -> Self {
        Self {
            corotinues: VecDeque::new(),
        }
    }

    fn add(&mut self, coroutine: Pin<Box<dyn Coroutine<(), Yield = (), Return = ()>>>) {
        self.corotinues.push_back(coroutine);
    }

    fn poll(&mut self) {
        println!("Polling {} coroutines", self.corotinues.len());
        let mut coroutine = self.corotinues.pop_front().unwrap();
        match coroutine.as_mut().resume(()) {
            CoroutineState::Yielded(_) => {
                self.corotinues.push_back(coroutine);
            }
            CoroutineState::Complete(_) => {}
        }
    }
}

struct SleepCoroutine {
    pub start: Instant,
    pub duration: std::time::Duration,
}

impl SleepCoroutine {
    fn new(duration: std::time::Duration) -> Self {
        Self {
            start: Instant::now(),
            duration,
        }
    }
}

impl Coroutine<()> for SleepCoroutine {
    type Yield = ();

    type Return = ();

    fn resume(self: Pin<&mut Self>, _: ()) -> CoroutineState<Self::Yield, Self::Return> {
        if self.start.elapsed() >= self.duration {
            CoroutineState::Complete(())
        } else {
            CoroutineState::Yielded(())
        }
    }
}

impl Future for SleepCoroutine {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self).resume(()) {
            CoroutineState::Yielded(_) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            CoroutineState::Complete(_) => Poll::Ready(()),
        }
    }
}

fn main() {
    let mut executor = Executor::new();
    for _ in 1..=3 {
        let coroutine = SleepCoroutine::new(std::time::Duration::from_secs(1));
        executor.add(Box::pin(coroutine));
    }
    let start = Instant::now();
    while !executor.corotinues.is_empty() {
        executor.poll();
    }
    println!("Took {:?}", start.elapsed());
}
