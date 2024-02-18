use std::error::Error;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

use crate::protocol::Packet;

// NOTE(omni): this may be upgraded to an enum later to support more global
//             server events.
struct ServerShutdown;

pub struct ZenaServer {
    /// Used to inform all sessions that the server is shutting down
    /// and so they should send a shutdown message.
    broadcast: tokio::sync::broadcast::Sender<ServerShutdown>,
}

impl Default for ZenaServer {
    fn default() -> Self {
        Self::new()
    }
}

impl ZenaServer {
    pub fn new() -> Self {
        Self {
            broadcast: tokio::sync::broadcast::Sender::new(100),
        }
    }

    pub async fn start_server(&self, addr: String) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(&addr).await?;

        tracing::info!("Server running on {addr}");

        // TODO: use the channel back
        let (tx, _rx) = broadcast::channel(100);

        loop {
            let (stream, addr) = listener.accept().await?;

            tracing::trace!("connection from Addr {addr}");
            let tx = tx.clone();
            tokio::spawn(async move {
                tracing::info!("Accepted a new connection");
                handle_connection(stream, tx).await;
            });
        }
    }
}

/// Takes a new socket and begins to listen for data with the intention of turning it into a
/// session and then receiving authentication data.
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
