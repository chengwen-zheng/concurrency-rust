use anyhow::Result;
use std::{io, net::SocketAddr};
use tokio::{io::AsyncWriteExt, net::TcpListener};
use tracing::{info, warn};

const BUFFER_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // build a listener
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    // accept connections
    loop {
        let (socket, raddr) = listener.accept().await?;
        info!("Accepted connection from: {}", raddr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_connect(socket, raddr).await {
                warn!("Error processing conn with {}: {:?}", raddr, e);
            }
        });
    }
}

async fn process_redis_connect(mut stream: tokio::net::TcpStream, raddr: SocketAddr) -> Result<()> {
    // readable
    loop {
        stream.readable().await?;
        let mut buffer = Vec::with_capacity(BUFFER_SIZE);
        match stream.try_read_buf(&mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                // process the buffer
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buffer);
                info!("Received: {:?}", line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    warn!("Connection {} closed", raddr);
    Ok(())
}
