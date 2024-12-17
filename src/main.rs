use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt,AsyncReadExt};
use std::io;
use std::io::{Result, Error};
use std::string::String;

async fn process_socket(mut stream: TcpStream) -> Result<String> {
    let _res = stream.write_all(b"Just Because!\n").await?;
    stream.readable().await?;
    let mut buffer = [0; 100];
    let _bytes_read = stream.read(&mut buffer).await?;
    let string = String::from_utf8_lossy(&buffer).to_owned().to_string();
    Ok(string)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Just Because");
    loop {
        let (stream, address) = listener.accept().await?;
        println!("Connected {}", address);
        let result = process_socket(stream).await.unwrap();
        println!("{}", result);
    }
}
