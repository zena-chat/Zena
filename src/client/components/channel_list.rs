use egui::{Label, RichText, Sense, Ui};

use crate::model::{Channel, ChannelId};

/// Renders a list of Channels with their ID and name.
/// The currently selected channel should be shown in bold and clicking
/// other channels will switch to that channel.
pub fn draw_channel_list(
    ui: &mut Ui,
    channels: &[Channel],
    selected_channel: &mut Option<ChannelId>,
) {
    channels.iter().for_each(|ch| {
        let text = format!("Channel {}: {}", ch.id, ch.name);
        let resp = match selected_channel {
            Some(id) if ch.id == *id => ui.label(RichText::new(text).strong()),
            _ => ui.add(Label::new(text).sense(Sense::click())),
        };
        if resp.clicked() {
            *selected_channel = Some(ch.id)
        }
    });
}
