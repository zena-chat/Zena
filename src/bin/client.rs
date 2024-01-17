//! Entrypoint for running the Client

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use zena::client::{app::ZenaApp, core::PlaceholderStore};

fn main() -> Result<(), eframe::Error> {
    pretty_env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native("Zena", options, Box::new(|cc| {
        Box::new(ZenaApp::<PlaceholderStore>::new(cc))
    }))
}
