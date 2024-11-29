use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use std::pin::Pin;

trait SymmetricCoroutine {
    type Input;
    type Output;

    fn resume_with_input(self: Pin<&mut Self>, input: Self::Input) -> Self::Output;
}

struct ReadCoroutine {
    lines: io::Lines<BufReader<File>>,
}

struct WriteCoroutine {
    pub file_handle: File,
}

impl ReadCoroutine {
    fn new(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines();

        Ok(Self { lines })
    }
}

impl WriteCoroutine {
    fn new(path: &str) -> io::Result<Self> {
        let file_handle = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self { file_handle })
    }
}

impl SymmetricCoroutine for ReadCoroutine {
    type Input = ();
    type Output = Option<i32>;

    fn resume_with_input(mut self: Pin<&mut Self>, _input: Self::Input) -> Self::Output {
        if let Some(Ok(line)) = self.lines.next() {
            line.parse::<i32>().ok()
        } else {
            None
        }
    }
}

impl SymmetricCoroutine for WriteCoroutine {
    type Input = i32;
    type Output = ();

    fn resume_with_input(mut self: Pin<&mut Self>, input: Self::Input) -> Self::Output {
        writeln!(self.file_handle, "{}", input).unwrap();
    }
}

fn main() -> io::Result<()> {
    let mut reader = ReadCoroutine::new("numbers.txt")?;
    let mut writer = WriteCoroutine::new("output.txt")?;

    loop {
        let number = Pin::new(&mut reader).resume_with_input(());
        if let Some(num) = number {
            Pin::new(&mut writer).resume_with_input(num);
        } else {
            break;
        }
    }
    Ok(())
}
