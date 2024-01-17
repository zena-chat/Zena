use super::{
    components::channel_list::draw_channel_list,
    core::{ClientCore, CoreAction},
    db::Db,
};
use crate::model::{Channel, ChannelId, Message};
use egui::mutex::Mutex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{mpsc::Sender, Arc},
};

/// This is the top-most struct representing the Zena client GUI application.
pub struct ZenaApp {
    /// Sends [CoreAction]s over a channel to a background thread.
    tx: Sender<CoreAction>,
    /// UI-local state. Can be mutated by egui
    ui_state: UIState,
    /// Data shared between the UI thread and the data thread (core)
    data: Arc<Mutex<CoreData>>,
}

impl ZenaApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let db = Db::init().unwrap(); // FIXME(josh): handle error

        let (tx, rx) = std::sync::mpsc::channel();

        let frame = cc.egui_ctx.clone();

        let core_data = Arc::new(Mutex::new(CoreData::default()));

        let mut core = ClientCore {
            db,
            data: core_data.clone(),
            frame,
        };
        std::thread::spawn(move || {
            // Start 'core' here.
            while let Ok(action) = rx.recv() {
                println!("Received {action:?}");
                core.handle_action(action)
            }
        });

        // prepopulate channels
        tx.send(CoreAction::FetchChannels).unwrap();

        Self {
            tx,
            ui_state: Default::default(),
            data: core_data,
        }
    }
}

/// Holds UI-specific state such as checkbox toggles, selected tab, etc
///
/// This should be serializable so we can store the user's current displayed state
/// and instantly resume from that view when they re-open the app.
#[derive(Serialize, Deserialize)]
pub struct UIState {
    new_channel_name: String,
    current_channel: Option<ChannelId>,
}
impl Default for UIState {
    fn default() -> Self {
        Self {
            new_channel_name: "General".to_string(),
            current_channel: None,
        }
    }
}

/// In-memory store of data that has been queried from the database.
///
/// Only data relevant to chat/messenging should be stored in here. Roughly anything
/// that might be represented in the database or otherwise derived from data in the
/// db can live here. For data that is specific to the UI implementation see [UIState]
/// 
/// The UI can access this data in order to render channel lists, online members,
/// chat messages, etc without having to query the database directly.
#[derive(Default)]
pub struct CoreData {
    pub channels: Vec<Channel>,
    pub messages: HashMap<Channel, Vec<Message>>,
}

impl eframe::App for ZenaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Zena Client");
            ui.horizontal(|ui| {
                let name_label = ui.label("Channel name: ");
                ui.text_edit_singleline(&mut self.ui_state.new_channel_name)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                if ui.button("Click to create new channel").clicked() {
                    self.tx
                        .send(CoreAction::CreateChannel(
                            self.ui_state.new_channel_name.clone(),
                        ))
                        .unwrap();
                }
                if ui.button("Clear all channels").clicked() {
                    self.tx.send(CoreAction::DeleteAllChannels).unwrap();
                }
            });
            ui.spacing();
            ui.heading("Channels");

            let data = self.data.lock(); // hold a lock on the CoreData until we've finished painting
            draw_channel_list(ui, &data.channels, &mut self.ui_state.current_channel);
        });
    }
}
