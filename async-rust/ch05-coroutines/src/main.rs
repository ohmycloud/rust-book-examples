#![feature(coroutines)]
#![feature(coroutine_trait)]

use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::time::Instant;

use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

fn append_number_to_file(n: i32) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("numbers.txt")?;
    writeln!(file, "{}", n)?;

    Ok(())
}

struct WriteCoroutine {
    pub file_handle: File,
}

impl WriteCoroutine {
    fn new(path: &str) -> io::Result<Self> {
        let file_handle = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self { file_handle })
    }
}

impl Coroutine<i32> for WriteCoroutine {
    type Yield = ();

    type Return = ();

    fn resume(mut self: Pin<&mut Self>, arg: i32) -> CoroutineState<Self::Yield, Self::Return> {
        writeln!(self.file_handle, "{}", arg).unwrap();
        CoroutineState::Yielded(())
    }
}

fn normal_thread() {
    let mut rng = rand::thread_rng();
    let numbers: Vec<i32> = (0..200000).map(|_| rng.r#gen()).collect();

    let start = Instant::now();
    for &number in &numbers {
        if let Err(e) = append_number_to_file(number) {
            eprintln!("Failed to write to file: {}", e);
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in file opeartions is: {:?}", duration);
}

fn with_coroutine() -> Result<(), Box<dyn std::error::Error>> {
    let mut coroutine = WriteCoroutine::new("numbers-coroutines.txt")?;
    let mut rng = rand::thread_rng();
    let numbers: Vec<i32> = (0..200000).map(|_| rng.r#gen()).collect();

    let start = Instant::now();
    for &number in &numbers {
        Pin::new(&mut coroutine).resume(number);
    }
    let duration = start.elapsed();
    println!("Time elapsed in file opeartions is: {:?}", duration);
    Ok(())
}

fn main() -> io::Result<()> {
    with_coroutine();
    Ok(())
}
