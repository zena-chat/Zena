//! How we communicate with the server from within the client core

use std::net::SocketAddr;

use tokio::net::TcpStream;

use crate::protocol::Payload;

// TODO: get a TCP connection I can shoot things over
struct ConnectionWrapper;

impl ConnectionWrapper {
    async fn send_msg_to_server<'a, P: Payload<'a>>(&self, payload: P) -> Result<(), NetErr> {
        let bytes = bincode::serialize(&payload).unwrap(); // TODO: handle error
        let mut stream = TcpStream::connect("127.0.0.1:7777").await.unwrap();
        // bincode::serialize_into(stream, &payload).unwrap();
        Ok(())
    }
}

/// TODO(josh): this is a placeholder
enum NetErr {}
