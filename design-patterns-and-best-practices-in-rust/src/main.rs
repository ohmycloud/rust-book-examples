use std::io::Read;

mod calculator;

fn main() {
    println!("Hello, world!");
}

fn find_block(_message: &[u8], _block_id: u64) -> (usize, usize) {
    todo!()
}

struct BlockReader {
    message: Vec<u8>,
    pos: usize,
    end: usize,
}

impl Read for BlockReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let remaining = self.end - self.pos;
        let to_read = remaining.min(buf.len());
        buf[..to_read].copy_from_slice(&self.message[self.pos..self.pos + to_read]);
        self.pos += to_read;
        Ok(to_read)
    }
}

fn new_block_reader(message: Vec<u8>, block_id: u64) -> BlockReader {
    let (pos, count) = find_block(&message, block_id);
    BlockReader {
        message,
        pos,
        end: pos + count,
    }
}
