use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_rustls::TlsAcceptor;
use rustls::server::ServerConfig;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};


fn load_tls_config() -> Result<ServerConfig, Box<dyn Error>> {
    let cert_file = File::open("server.crt")?;
    let mut cert_reader = BufReader::new(cert_file);
    
    let certs: Vec<CertificateDer> = rustls_pemfile::certs(&mut cert_reader)
        .collect::<Result<Vec<_>, _>>()?;

    let key_file = File::open("server.key")?;
    let mut key_reader = BufReader::new(key_file);

    let key: PrivateKeyDer = rustls_pemfile::private_key(&mut key_reader)?
        .ok_or("No private key found in key.pem")?;

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    Ok(config)
}

async fn process(mut socket: tokio_rustls::server::TlsStream<TcpStream>) -> Result<(), Box<dyn Error>> {
    let mut buf = [0u8; 1024];
    let n = socket.read(&mut buf).await?;

    if n == 0 { return Ok(()); }

    let msg = String::from_utf8_lossy(&buf[..n]);
    println!("Recived {}", msg);

    socket.write_all(b"Secure Hello!").await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = load_tls_config()?;
    let acceptor = TlsAcceptor::from(Arc::new(config));

    let listener = TcpListener::bind("10.38.75.248:7777").await?;
    println!("Secure Server started...");

    loop {
        let (stream, addr) = listener.accept().await?;
        let acceptor = acceptor.clone();

        println!("New connection: {}", addr);

        tokio::spawn(async move {
            match acceptor.accept(stream).await {
                Ok(tls_stream) => {
                    if let Err(e) = process(tls_stream).await {
                        eprintln!("Error processing: {}", e);
                    }
                }
                Err(e) => eprintln!("TLS Handshake error: {}", e),
            }
        });
    }
}