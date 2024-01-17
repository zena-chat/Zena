use std::sync::Arc;

use egui::{mutex::Mutex, Context};

use super::{app::CoreData, db::Db};

/// Commands that the GUI can send to the data thread.
/// By communicating via `CoreAction`s the GUI doesn't need to know how to do database queries
/// or network IO nor will it be blocked by such.
///
/// Any time you want to modify data such as channels, users, send chat messages,
/// or edit things, add a new variant to this enum.
#[derive(Debug)]
pub enum CoreAction {
    /// Fetch all channels and refresh the in-memory store in [CoreData]
    FetchChannels,
    /// Create a new channel on the server. Should update local storage AND talk to the server
    CreateChannel(String),
    /// Deletes all channels
    DeleteAllChannels,
}

pub struct ClientCore {
    /// Wrapper around a SQLite connection for performing queries
    pub db: Db,
    /// Data shared between UI and Data threads. For full details see [CoreData]
    pub data: Arc<Mutex<CoreData>>,
    /// Use this to force a repaint if we receive data on the network while the
    /// user is not interacting with the app.
    pub frame: Context,
}

impl ClientCore {
    pub fn handle_action(&mut self, action: CoreAction) {
        match action {
            CoreAction::FetchChannels => {
                let channels = self.db.get_all_channels().unwrap(); // FIXME: handle error
                let mut shared_data = self.data.lock();
                let _ = std::mem::replace(&mut shared_data.channels, channels); // replace the cached data with the new data
            }
            CoreAction::CreateChannel(name) => {
                self.db.create_channel(name).unwrap();
                self.handle_action(CoreAction::FetchChannels);
            }
            CoreAction::DeleteAllChannels => {
                self.db.delete_all_channels().unwrap();
                self.handle_action(CoreAction::FetchChannels);
            }
        };
        self.frame.request_repaint();
    }
}
