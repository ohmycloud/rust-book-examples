use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll as MioPoll, Token};
use std::error::Error;
use std::future::Future;
use std::io::{Read, Write};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

struct ServerFuture {
    server: TcpListener,
    poll: MioPoll,
}

impl Future for ServerFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut events = Events::with_capacity(1);
        let _ = self
            .poll
            .poll(&mut events, Some(Duration::from_millis(200)))
            .unwrap();
        for event in events.iter() {
            if event.token() == SERVER && event.is_readable() {
                let (mut stream, _) = self.server.accept().unwrap();
                let mut buffer = [0; 1024];
                let mut received_data = Vec::new();

                loop {
                    match stream.read(&mut buffer) {
                        Ok(n) if n > 0 => {
                            received_data.extend_from_slice(&buffer[..n]);
                        }
                        Ok(_) => {
                            break;
                        }
                        Err(e) => {
                            eprintln!("Error reading from stream: {}", e);
                        }
                    }
                }
                if !received_data.is_empty() {
                    let received_str = String::from_utf8_lossy(&received_data);
                    return Poll::Ready(received_str.to_string());
                }
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        }
        cx.waker().wake_by_ref();
        return Poll::Pending;
    }
}

fn main() {}
