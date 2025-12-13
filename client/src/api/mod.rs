use ::shared::Request;
use rustls::ClientConfig;
use rustls::pki_types::ServerName;
use rustls_pki_types::CertificateDer;
use std::io::{Cursor, Read, Write};
use std::sync::Arc;

pub mod cabinet;
pub mod direction;
pub mod journal;
pub mod laborant;
pub mod student;
pub mod summary;

const CERT_BYTES: &[u8] = include_bytes!("../../../certificates/rootCA.pem");
const SERVER_ADDR: &str = "217.60.2.146:7777";
const SERVER_NAME: &str = "217.60.2.146";

pub fn send_request(req: Request) -> anyhow::Result<serde_json::Value> {
    let mut root_store = rustls::RootCertStore::empty();
    let mut reader = Cursor::new(CERT_BYTES);
    let certs: Vec<CertificateDer> =
        rustls_pemfile::certs(&mut reader).collect::<Result<Vec<_>, _>>()?;

    for cert in certs {
        root_store.add(cert)?;
    }

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let server_name: ServerName =
        ServerName::try_from(SERVER_NAME).map_err(|_| anyhow::anyhow!("Invalid server name"))?;

    let conn = rustls::ClientConnection::new(Arc::new(config), server_name)?;
    let tcp = std::net::TcpStream::connect(SERVER_ADDR)
        .map_err(|e| anyhow::anyhow!("Failed to connect: {}", e))?;

    let mut tls = rustls::StreamOwned::new(conn, tcp);

    let body = serde_json::to_vec(&req)?;
    tls.write_all(&body)?;
    tls.flush()?;

    let mut buf = Vec::new();
    tls.read_to_end(&mut buf)?;

    let value: serde_json::Value = serde_json::from_slice(&buf)?;
    Ok(value)
}
