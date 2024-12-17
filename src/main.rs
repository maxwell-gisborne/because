use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt,AsyncReadExt};
use std::io;
use std::io::Result;
use std::string::String;

async fn process_socket(mut stream: TcpStream) -> Result<String> {
    let _res = stream.write_all(b"Just Because!\n").await?;
    stream.readable().await?;
    let mut buffer = [0; 100];
    let _bytes_read = stream.read(&mut buffer).await?;
    let string = String::from_utf8_lossy(&buffer).to_owned().to_string();
    Ok(string)
}

async fn run_server() -> Result<()>{
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Runing server");
    loop {
        let (stream, address) = listener.accept().await?;
        println!("Connected {}", address);
        let result = process_socket(stream).await.unwrap();
        println!("{}", result);
    }
}

enum ServerFoundStatus {
    Found,
    No,
}

async fn run_client() -> Result<ServerFoundStatus> {
    match TcpStream::connect("127.0.0.1:8080").await {
        Err(_) => {return Ok(ServerFoundStatus::No);},
        Ok(stream) => {println!("{}", process_socket(stream).await.unwrap());},
    };
    Ok(ServerFoundStatus::Found)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    match run_client().await.unwrap() {
        ServerFoundStatus::No => run_server().await,
        ServerFoundStatus::Found => Ok(())
    }
}
