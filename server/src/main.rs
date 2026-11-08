use anyhow::Context;
use dotenvy::dotenv;
use rustls::server::ServerConfig;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use shared::{Request, Resource};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::TlsAcceptor;

mod handlers;
use handlers::cabinet::handle_cabinet;
use handlers::direction::handle_direction;
use handlers::journal::handle_journal;
use handlers::json_err;
use handlers::laborant::handle_laborant;
use handlers::student::handle_student;
use handlers::summary::handle_summary;

fn load_tls_config() -> anyhow::Result<ServerConfig> {
    let cert_path = env::var("TLS_CERT_PATH").unwrap_or_else(|_| "./sec/server.crt".to_string());
    let cert_file = File::open(&cert_path)
        .with_context(|| format!("Failed to open TLS certificate file at '{}'", cert_path))?;
    let mut cert_reader = BufReader::new(cert_file);

    let certs: Vec<CertificateDer> =
        rustls_pemfile::certs(&mut cert_reader).collect::<Result<Vec<_>, _>>()?;

    let key_path = env::var("TLS_KEY_PATH").unwrap_or_else(|_| "./sec/server.key".to_string());
    let key_file = File::open(&key_path)
        .with_context(|| format!("Failed to open TLS private key file at '{}'", key_path))?;
    let mut key_reader = BufReader::new(key_file);

    let key: PrivateKeyDer =
        rustls_pemfile::private_key(&mut key_reader)?.context("no private key found")?;

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    Ok(config)
}

async fn process(
    mut socket: tokio_rustls::server::TlsStream<TcpStream>,
    pool: sqlx::Pool<sqlx::Postgres>,
) -> anyhow::Result<()> {
    let mut buf = [0u8; 1024];

    let n = socket.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }

    let response_json = match serde_json::from_slice::<Request>(&buf[..n]) {
        Ok(request) => {
            println!("Handling resource: {:?}", request.resource);

            match request.resource {
                Resource::Direction => handle_direction(&pool, request.action).await,
                Resource::Student => handle_student(&pool, request.action).await,
                Resource::Cabinet => handle_cabinet(&pool, request.action).await,
                Resource::Laborant => handle_laborant(&pool, request.action).await,
                Resource::Journal => handle_journal(&pool, request.action).await,
                Resource::Summary => handle_summary(&pool, request.action).await,
            }
        }
        Err(e) => json_err(&format!("Invalid JSON: {}", e)),
    };

    let response_bytes = serde_json::to_vec(&response_json)?;
    socket.write_all(&response_bytes).await?;

    socket.shutdown().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = load_tls_config()?;
    let pool = PgPoolOptions::new()
        .max_connections(9)
        .connect(&db_url)
        .await?;

    let acceptor = TlsAcceptor::from(Arc::new(config));
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "7777".to_string());
    let bind_addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&bind_addr).await?;
    println!("Secure Server started on {}...", bind_addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        let acceptor = acceptor.clone();

        let pool = pool.clone();

        println!("New connection: {}", addr);

        tokio::spawn(async move {
            match acceptor.accept(stream).await {
                Ok(tls_stream) => {
                    if let Err(e) = process(tls_stream, pool).await {
                        eprintln!("Error processing: {}", e);
                    }
                }
                Err(e) => eprintln!("TLS Handshake error: {}", e),
            }
        });
    }
}
