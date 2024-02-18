use std::error::Error;

use tokio::{io::AsyncWriteExt, net::TcpStream};
use zena::protocol::{message_types::NewMsg, Packet};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = TcpStream::connect("127.0.0.1:7777").await.unwrap();

    let data = Packet::NewMsg(NewMsg {
        channel_id: 123,
        contents: "Hey guys!".to_string(),
    });
    let packet = bincode::serialize(&data).unwrap();

    let len = socket.write(&packet).await.unwrap();
    println!("Write {len} bytes to socket");

    Ok(())
}
