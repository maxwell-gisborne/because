use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use std::io;

async fn process_socket(mut stream: TcpStream) {
    stream.write_all(b"Just Because!\n").await;
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Just Because");
    loop {
        let (stream, address) = listener.accept().await?;
        process_socket(stream).await;
    }
}
