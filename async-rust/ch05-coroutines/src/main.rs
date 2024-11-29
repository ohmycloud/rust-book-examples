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

fn main() -> io::Result<()> {
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

    Ok(())
}
