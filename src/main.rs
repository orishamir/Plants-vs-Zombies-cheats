#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::NativeOptions;
use egui::{self, ViewportBuilder};
use gui::MyApp;

mod game;
mod gui;
mod models;
mod toggleables;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1920.0, 1080.0])
            .with_transparent(true)
            .with_always_on_top(),
        // .with_decorations(false),
        // .with_fullscreen(true)
        // .with_mouse_passthrough(true) DOES NOT WORK! Workaround: https://github.com/emilk/egui/issues/2537#issuecomment-2255399952
        // .with_transparent(true),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            // cc.egui_ctx
            // .send_viewport_cmd(ViewportCommand::MousePassthrough(true));
            let thing = Box::<MyApp>::default();
            Ok(thing)
        }),
    )
}
