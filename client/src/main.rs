use std::io::Cursor;
use std::sync::Arc;
use std::error::Error;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rustls_pki_types::CertificateDer;
use rustls::ClientConfig;
use tokio_rustls::TlsConnector;
use rustls::pki_types::ServerName;


const CERT_BYTES: &[u8] = include_bytes!("../../certificates/rootCA.pem"); 


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut root_store = rustls::RootCertStore::empty();
    
    let mut reader = Cursor::new(CERT_BYTES);
    let certs: Vec<CertificateDer> = rustls_pemfile::certs(&mut reader).collect::<Result<Vec<_>, _>>()?;
    for cert in certs {
        root_store.add(cert)?;
    }

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    
    let connector = TlsConnector::from(Arc::new(config));

    let stream = TcpStream::connect("10.38.75.248:7777").await?;

    let domain = ServerName::try_from("10.38.75.248")?; 
    let mut socket = connector.connect(domain, stream).await?;

    socket.write_all(b"Hello Secure World!").await?;

    let mut buf = [0u8; 1024];
    let n = socket.read(&mut buf).await?;
    println!("Response: {}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}
