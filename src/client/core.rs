use std::{collections::BTreeSet, sync::Arc};

use egui::mutex::Mutex;

use crate::model::Channel;

use super::{db::Db, store::Store};

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

/// Top-most struct that sits in the Data thread. Holds data needed to perform IO
/// to the outside world as well as mutate the in-memory data that the UI needs to
/// render.
pub struct ClientCore {
    /// Wrapper around a SQLite connection for performing queries
    pub db: Db,
    pub store: Arc<Mutex<Store>>,
    /// Use this to force a repaint if we receive data on the network while the
    /// user is not interacting with the app.
    pub frame: egui::Context, // TODO: abstract this so that we don't own a egui specific struct
}

impl ClientCore {
    pub fn handle_action(&mut self, action: CoreAction) {
        match action {
            CoreAction::FetchChannels => {
                let channels: BTreeSet<Channel> =
                    self.db.get_all_channels().unwrap().into_iter().collect(); // FIXME: handle error
                let mut shared_data = self.store.lock();
                let _ = std::mem::replace(&mut shared_data.channels, channels); // replace the cached data with the new data
            }
            CoreAction::CreateChannel(name) => {
                // 1. Persist it in the local database
                let _channel = self.db.create_channel(name).unwrap();

                // 2. Insert it into the Store
                // let mut store = self.store.lock();
                // store.channels.insert(channel);
                // drop(store); // drop the mutex lock

                // TODO: 3. Send it to the server

                // Request a resync for channels
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
