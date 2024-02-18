use std::{env, error::Error};

use zena::server::ZenaServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // init our tracing subsriber that logs traces emitted by the server. they can be emitted
    // from any thread or task.
    tracing_subscriber::fmt().init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7777".to_string());

    let zena_server = ZenaServer::new();
    zena_server.start_server(addr).await
}
