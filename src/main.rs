#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::NativeOptions;
use egui::{self, ViewportBuilder};
use gui::MyApp;

use crate::{
    entities_loader::EntitiesLoader,
    game::GameProcess,
    models::{Card, Griditem},
};

mod entities_loader;
mod game;
mod gui;
mod models;
mod overlay_gui;
mod toggleables;

fn _main() {
    let proc = GameProcess::default();
    let ents = EntitiesLoader::load(&proc).unwrap();
    println!("{:#?}", ents.lawnmowers);
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_always_on_top()
            .with_transparent(true),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}
