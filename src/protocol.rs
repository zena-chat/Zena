//! The data we want to send between server <-> client

use crate::model::MessageId;

/// A new chat message from a client
struct NewMsg {
  channel_id: i64,
  contents: String,
}

/// Client request to delete a message
struct DeleteMsg {
  msg_id: MessageId,
}

/// Client request to edit a message
struct EditMsg {
  msg_id: MessageId,
  updated_contents: String,
}