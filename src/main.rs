// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::NativeOptions;
use egui::{self, ViewportBuilder};
use pvz_sdk::gui::MyApp;
use pvz_sdk::{
    EntitiesLoader, Popcapgame,
    models::{Coin, CoinContent},
};

fn main() {
    // let proc = GameProcess::default();
    // let addr = proc.base_module.base_address();
    // let ents = proc
    //     .proc
    //     .read_mem_chain::<u32>(vec![
    //         addr, 0x32f39c, 0x540, 0x48c, 0x0, 0x3dc, 0x4, 0x0, 0xa4,
    //     ])
    //     .unwrap();
    // println!("{:x}", ents)
    // let proc = GameProcess::init().expect("problem with game process");
    // loop {
    //     let ents = EntitiesLoader::load(&proc).unwrap();
    //     println!("{:#?}", ents.cards);
    // }
    let proc = Popcapgame::default();

    let mut ents = EntitiesLoader::load(&proc).unwrap();
    for mut coin in ents.coins {
        let CoinContent::Sun = coin.entity.content else {
            println!("bomboclat");
            continue;
        };

        coin.entity = Coin {
            display_pos_x: coin.entity.display_pos_x,
            display_pos_y: coin.entity.display_pos_y,
            is_deleted: coin.entity.is_deleted,
            destination_y: coin.entity.destination_y,
            age_since_spawned: coin.entity.age_since_spawned,
            age_since_reached_destination: coin.entity.age_since_reached_destination,
            content: CoinContent::Diamond,
        };
        coin.write_entity(&proc);
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
