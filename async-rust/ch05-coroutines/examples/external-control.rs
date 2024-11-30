#![feature(coroutines, coroutine_trait)]
use rand::Rng;
use std::{
    ops::{Coroutine, CoroutineState},
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

struct RandCoRoutine {
    pub value: u8,
    pub live: bool,
}

impl RandCoRoutine {
    fn new() -> Self {
        let mut coroutine = Self {
            value: 0,
            live: true,
        };

        coroutine.generate();
        coroutine
    }
    fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        self.value = rng.gen_range(0..=10);
    }
}

impl Coroutine<()> for RandCoRoutine {
    type Yield = u8;

    type Return = ();

    fn resume(mut self: Pin<&mut Self>, _: ()) -> CoroutineState<Self::Yield, Self::Return> {
        self.generate();
        CoroutineState::Yielded(self.value)
    }
}

struct NestingFuture {
    inner: Pin<Box<dyn Future<Output = ()> + Send>>,
}

impl Future for NestingFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.inner.as_mut().poll(cx) {
            Poll::Ready(_) => Poll::Ready(()),
            Poll::Pending => Poll::Pending,
        }
    }
}

fn send_coroutines_over_threads() {
    let (tx, rx) = std::sync::mpsc::channel::<RandCoRoutine>();
    let _thread = std::thread::spawn(move || {
        loop {
            let mut coroutine = match rx.recv() {
                Ok(coroutine) => coroutine,
                Err(_) => break,
            };
            match Pin::new(&mut coroutine).resume(()) {
                CoroutineState::Yielded(result) => {
                    println!("Coroutine yielded: {}", result);
                }
                CoroutineState::Complete(_) => {
                    panic!("Coroutine should not complete");
                }
            }
        }
    });
    std::thread::sleep(Duration::from_secs(1));
    tx.send(RandCoRoutine::new()).unwrap();
    tx.send(RandCoRoutine::new()).unwrap();
    std::thread::sleep(Duration::from_secs(1));
}

fn main() {
    let mut coroutines = Vec::new();
    for _ in 0..10 {
        coroutines.push(RandCoRoutine::new());
    }
    let mut total: u32 = 0;

    loop {
        let mut all_dead = true;
        for mut coroutine in coroutines.iter_mut() {
            if coroutine.live {
                all_dead = false;
                match Pin::new(&mut coroutine).resume(()) {
                    CoroutineState::Yielded(result) => total += result as u32,
                    CoroutineState::Complete(_) => {
                        panic!("Coroutine should not complete!")
                    }
                }
                if coroutine.value < 9 {
                    coroutine.live = false;
                }
            }
        }

        if all_dead {
            break;
        }
    }
    println!("Total: {}", total);

    send_coroutines_over_threads();
}
