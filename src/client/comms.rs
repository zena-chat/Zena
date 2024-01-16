//! How we communicate with the server from within the client core

use std::io::{Write, self};

trait Payload {
  /// Serialize a payload to bytes and write it to a buffer
  fn serialize(&self, buf: &mut impl Write) -> io::Result<()>;
}

async fn send_msg_to_server<P: Payload>() -> Result<(), NetErr> {
  Ok(())
}

/// TODO(josh): this is a placeholder
enum NetErr {}