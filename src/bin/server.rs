use std::{env, error::Error, sync::Arc};

use tokio::{net::TcpListener, sync::Mutex};
use zena::server::ServerPeers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // init our tracing subsriber that logs traces emitted by the server. they can be emitted
    // from any thread or task.
    tracing_subscriber::fmt().init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7777".to_string());

    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("Server running on {addr}");

    let state = Arc::new(Mutex::new(ServerPeers::default()));

    loop {
        let (stream, addr) = listener.accept().await?;

        tokio::spawn(async move {
            tracing::trace!("Accepted a new connection");
        });
    }
}
