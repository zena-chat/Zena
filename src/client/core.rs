use std::sync::Arc;

use egui::{mutex::Mutex, Context};

use super::{app::CoreData, db::Db};

#[derive(Debug)]
pub enum CoreAction {
    /// Instructs the data thread to fetch all channels
    FetchChannels,
    CreateChannel(String),
    DeleteAllChannels,
}

pub struct ClientCore {
    pub db: Db,
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
