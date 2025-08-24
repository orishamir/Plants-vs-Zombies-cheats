// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::NativeOptions;
use egui::{self, ViewportBuilder};
use gui::MyApp;

use crate::{entities_loader::EntitiesLoader, game::GameProcess, models::Zombie};

mod entities_loader;
mod game;
mod gui;
mod models;
mod overlay_gui;
mod toggleables;

fn main() {
    // let proc = GameProcess::default();
    // println!("{:#x?}", proc.read_at::<Zombie>(0x22530628))
    let proc = GameProcess::init().expect("problem with game process");
    loop {
        let ents = EntitiesLoader::load(&proc).unwrap();
        println!("{:#?}", ents.cards);
    }
}

fn _main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_always_on_top()
            .with_transparent(true)
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_title("Plants vs. Zombies cheats"),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}
