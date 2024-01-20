use std::collections::{BTreeSet, HashMap};

use crate::model::{Channel, ChannelId, Message};

/// In-memory store of data that has been queried from the database.
///
/// Only data relevant to chat/messenging should be stored in here. Roughly anything
/// that might be represented in the database or otherwise derived from data in the
/// db can live here. For data that is specific to the UI implementation see [UIState]
///
/// The UI can access this data in order to render channel lists, online members,
/// chat messages, etc without having to query the database directly.
pub struct Store {
    /// Channels are ordered by `priority`
    pub(crate) channels: BTreeSet<Channel>,
    pub(crate) messages: HashMap<ChannelId, Vec<Message>>,
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Store {
    pub fn new() -> Self {
        Self {
            channels: Default::default(),
            messages: Default::default(),
        }
    }
    pub fn all_channels(&self) -> impl Iterator<Item = &Channel> {
        self.channels.iter()
    }

    pub fn messages_for_channel(&self, channel_id: ChannelId) -> &[Message] {
        if let Some(msgs) = self.messages.get(&channel_id) {
            msgs
        } else {
            &[]
        }
    }
}
