use std::{collections::HashMap, net::SocketAddr};

pub struct ConnectedClient {
    rx: tokio::sync::mpsc::Receiver<Vec<u8>>,
}

#[derive(Default)]
pub struct ServerPeers {
    peers: HashMap<SocketAddr, tokio::sync::mpsc::Sender<Vec<u8>>>,
}
