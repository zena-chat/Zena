//! The data we want to send between server <-> client

use serde::{Deserialize, Serialize};

use self::message_types::{DeleteMsg, EditMsg, NewMsg, Startup};

pub trait Payload<'a>: serde::Serialize + serde::Deserialize<'a> {}

#[derive(Debug, Serialize, Deserialize)]
pub enum Packet {
    Startup(Startup),
    NewMsg(NewMsg),
    DeleteMsg(DeleteMsg),
    EditMsg(EditMsg),
}

pub mod message_types {
    use crate::model::MessageId;
    use serde::{Deserialize, Serialize};

    use super::ClientMetadata;

    /// A new chat message from a client
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NewMsg {
        pub channel_id: i64,
        pub contents: String,
    }

    /// Client request to delete a message
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeleteMsg {
        msg_id: MessageId,
    }

    /// Client request to edit a message
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EditMsg {
        msg_id: MessageId,
        updated_contents: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Startup {
        client_metadata: ClientMetadata,
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientMetadata {
    version: u32,
}
impl ClientMetadata {
    fn new(version: u32) -> Self {
        Self { version }
    }
}

#[derive(Serialize, Deserialize)]
struct ServerMetadata {
    version: u32,
}
