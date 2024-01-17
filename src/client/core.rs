use std::{sync::Arc, collections::HashMap};

use egui::{mutex::Mutex, Context};

use crate::model::{Channel, Message};

use super::{app::CoreData, db::Db};

#[derive(Debug)]
pub enum CoreAction {
    /// Instructs the data thread to fetch all channels
    FetchChannels,
    CreateChannel(String),
    DeleteAllChannels,
}

pub trait Store: Default {
    // fn all_channels(&self) -> impl Iterator<Item = Channel>;
    fn all_channels(&self) -> Vec<Channel>;
}


///
/// Note that we made the members private
pub struct InMemoryStore {
    db: Db,
    channels: Vec<Channel>,
    messages: HashMap<Channel, Vec<Message>>,
}
pub struct PlaceholderStore;
pub struct NetworkStore;

impl Store for PlaceholderStore {
    fn all_channels(&self) -> Vec<Channel>  {
        vec![Channel { id: 1, name: "general".to_string(), created_at: 9999, updated_at: 9999 }]
    }
}

impl InMemoryStore {
    pub fn new() -> Self {
        let db = Db::init().unwrap(); // FIXME(josh): handle error

        Self {
            db,
            ..Default::default()
        }
    } 
}
impl Store for InMemoryStore {
    fn all_channels(&self) -> Vec<Channel> {
        self.db.get_all_channels().unwrap()
    }
}

pub struct ClientCore<S: Store> {
    // pub db: Db,
    pub store: S,
    /// Use this to force a repaint if we receive data on the network while the
    /// user is not interacting with the app.
    pub frame: Context,
}

impl<S: Store> ClientCore<S> {
    pub fn handle_action(&mut self, action: CoreAction) {
        match action {
            CoreAction::FetchChannels => {
                let channels = self.store.all_channels();
                let mut shared_data = self.store.lock();
                let _ = std::mem::replace(&mut shared_data.channels, channels); // replace the cached data with the new data
            }
            _ => () // TEMP: do nothing

            // CoreAction::CreateChannel(name) => {
            //     self.db.create_channel(name).unwrap();
            //     self.handle_action(CoreAction::FetchChannels);
            // }
            // CoreAction::DeleteAllChannels => {
            //     self.db.delete_all_channels().unwrap();
            //     self.handle_action(CoreAction::FetchChannels);
            // }
        };
        self.frame.request_repaint();
    }
}
