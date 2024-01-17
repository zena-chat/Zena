//! The data we want to send between server <-> client

use serde::{Deserialize, Serialize};

pub trait Payload<'a>: serde::Serialize + serde::Deserialize<'a> {}

pub mod message_types {
    use crate::model::MessageId;
    use serde::{Deserialize, Serialize};

    use super::ClientMetadata;

    /// A new chat message from a client
    #[derive(Serialize, Deserialize)]
    struct NewMsg {
        channel_id: i64,
        contents: String,
    }

    /// Client request to delete a message
    #[derive(Serialize, Deserialize)]
    struct DeleteMsg {
        msg_id: MessageId,
    }

    /// Client request to edit a message
    #[derive(Serialize, Deserialize)]
    struct EditMsg {
        msg_id: MessageId,
        updated_contents: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Startup {
        client_metadata: ClientMetadata,
    }
}

#[derive(Serialize, Deserialize)]
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
