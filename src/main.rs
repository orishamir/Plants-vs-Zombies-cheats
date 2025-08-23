// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::NativeOptions;
use egui::{self, ViewportBuilder};
use gui::MyApp;

use crate::{entities_loader::EntitiesLoader, game::GameProcess};

mod entities_loader;
mod game;
mod gui;
mod models;
mod overlay_gui;
mod toggleables;

fn _main() {
    let proc = GameProcess::init().expect("problem with game process");
    loop {
        let ents = EntitiesLoader::load(&proc).unwrap();
        println!("{:#?}", ents.cards);
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_always_on_top()
            .with_transparent(true)
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_title("Plants vs. Zombies cheats"),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}
