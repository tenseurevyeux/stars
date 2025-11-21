use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;


#[tokio::main] 
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket  = TcpStream::connect("10.38.75.248:7777").await?;

    socket.write_all(b"Hello?").await?;

    let mut buf = [0u8; 1024];
    let n = socket.read(&mut buf).await?;

    let response = String::from_utf8_lossy(&buf[..n]);
    println!("response: {}", response);

    Ok(())
}