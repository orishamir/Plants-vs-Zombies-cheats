#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::NativeOptions;
use egui::{self, ViewportBuilder};
use gui::MyApp;

use crate::{
    game::GameProcess,
    models::{entities::Entities, zombie::Zombie},
};

mod game;
mod gui;
mod models;
mod toggleables;

fn main() {
    let proc = GameProcess::default();
    // [[[[[[[<popcapgame1.exe>+]+]+]+0]+3dc]+4]+0]+a4
    let ents = proc
        .read_with_base_addr::<Entities>(&[0x32f39c, 0x540, 0x48c, 0x0, 0x3dc, 0x4, 0x0, 0xa4])
        .expect("oh no");
    println!("{}", size_of::<Zombie>());
    let x = proc
        .read::<Zombie>(&[ents.zombies_ptr as usize])
        .expect("oh no");
    println!("{x:#?}")
}

fn _main2() -> eframe::Result {
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
