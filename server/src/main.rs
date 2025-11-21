use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::error::Error;

async fn process(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf= [0u8; 1024];
    let n = socket.read(&mut buf).await?;

    if n == 0 {
        return Ok(());
    }

    let recieve_data = String::from_utf8_lossy(&buf[..n]);
    println!("msg: {}", recieve_data);

    socket.write_all(b"successful conn").await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("10.38.75.248:8080").await?;
    println!("Server started");
    loop { 
        let (socket, addr) = listener.accept().await?;
        println!("New user connected: {}", addr);
        
        tokio::spawn(async move {
            if let Err(e) = process(socket).await {
                eprintln!("Error \n {}", e); 
            }
        });
    }
}