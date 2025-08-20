#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::NativeOptions;
use egui::{self, ViewportBuilder};
use gui::MyApp;

mod entities_loader;
mod game;
mod gui;
mod models;
mod overlay_gui;
mod toggleables;

fn _main() {
    // let proc = GameProcess::default();
    // loop {
    //     let loader = entities_loader::EntitiesLoader::load(&proc).unwrap();
    //     loader.plants.iter().for_each(|plant| {
    //         println!("hello {plant:#?}");
    //     });
    //     thread::sleep(Duration::from_millis(500));
    //     print!("\x1B[2J\x1B[1;1H");
    // }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_transparent(true)
            .with_always_on_top(),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}
