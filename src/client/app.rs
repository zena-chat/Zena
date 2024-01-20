use super::{
    components::channel_list::draw_channel_list,
    core::{ClientCore, CoreAction},
    db::Db,
    store::Store,
};
use crate::model::ChannelId;
use egui::mutex::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::{mpsc::Sender, Arc};

/// This is the top-most struct representing the Zena client GUI application.
pub struct ZenaApp {
    /// Sends [CoreAction]s over a channel to a background thread.
    tx: Sender<CoreAction>,
    /// UI-local state. Can be mutated by egui
    ui_state: UIState,

    store: Arc<Mutex<Store>>,
}

impl ZenaApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let (tx, rx) = std::sync::mpsc::channel();

        let frame = cc.egui_ctx.clone();

        let store = Arc::new(Mutex::new(Store::new()));

        let db = Db::init().unwrap();
        let mut core = ClientCore {
            db,
            store: store.clone(),
            frame,
        };
        // move the ClientCore into a separate thread
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
            store,
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

impl eframe::App for ZenaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let store = self.store.lock();

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

            // let data = self.store.lock(); // hold a lock on the CoreData until we've finished painting
            draw_channel_list(ui, store.all_channels(), &mut self.ui_state.current_channel);
        });
    }
}
