use std::{collections::HashMap, net::SocketAddr};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::broadcast,
};

use crate::protocol::Packet;

pub struct ConnectedClient {
    name: String,
    rx: tokio::sync::mpsc::Receiver<Vec<u8>>,
}

#[derive(Default)]
pub struct ServerPeers {
    peers: HashMap<SocketAddr, ConnectedClient>,
}

pub struct ZenaServer {
    // broadcast: tokio::sync::broadcast:: ;
}

pub async fn handle_connection(mut socket: TcpStream, broadcaster: broadcast::Sender<Vec<u8>>) {
    let mut send_to_client = broadcaster.subscribe();

    let mut buf = vec![0; 1024];

    loop {
        tokio::select! {
                    // We received data on the socket FROM the client
                    result = socket.read(&mut buf) => {
                        let bytes = result.unwrap();
                        if bytes == 0 {
                            break;
                        }
                        tracing::info!("Received {bytes} bytes");
                        let data: Packet = bincode::deserialize(&buf).unwrap();
                        tracing::info!("Received data: {data:?}");
                        match data {
                            Packet::NewMsg(_msg) => {
                                let ownedmsg = buf.clone();
                                // This is pretty dumb. we dont really want to broadcast the exact
                                // payload back to every single client but for now it will do.
                                let _ = broadcaster.send(ownedmsg);
                            },
                            Packet::Startup(_) => todo!(),
                            Packet::DeleteMsg(_) => todo!(),
                            Packet::EditMsg(_) => todo!(),
        }
                        buf.clear();
                    }

                    // The server needs to send a message TO this client
                    to_send = send_to_client.recv() => {
                        if let Ok(data) = to_send {
                            socket.write(&data).await.unwrap();
                        }
                    }
                }
    }
}
