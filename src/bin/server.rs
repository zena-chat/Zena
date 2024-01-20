use std::{env, error::Error, sync::Arc};

use tokio::{
    net::TcpListener,
    sync::{broadcast, Mutex},
};
use tracing::trace;
use zena::server::{handle_connection, ServerPeers};

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

    let _state = Arc::new(Mutex::new(ServerPeers::default()));

    let (tx, _rx) = broadcast::channel(100);

    loop {
        let (stream, addr) = listener.accept().await?;

        trace!("connection from Addr {addr}");
        let tx = tx.clone();
        tokio::spawn(async move {
            tracing::info!("Accepted a new connection");
            handle_connection(stream, tx).await;
        });
    }
}
