use log::warn;
use pvz_sdk::{
    EntitiesLoader, Popcapgame,
    models::{Coin, CoinContent},
};

fn main() {
    unsafe { std::env::set_var("RUST_LOG", "trace") };
    env_logger::init();

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
    let proc = Popcapgame::init().unwrap();

    let ents = EntitiesLoader::load(&proc).unwrap();
    // println!("{:#?}", ents.cards);
    // println!("{:#?}", ents.coins);
    println!("{:#?}", ents.griditems);
    // println!("{:#?}", ents.lawnmowers);
    // for mut coin in ents.coins {
    //     let CoinContent::Sun = coin.entity.content else {
    //         warn!("bomboclat");
    //         continue;
    //     };

    //     coin.entity = Coin {
    //         display_pos_x: coin.entity.display_pos_x,
    //         display_pos_y: coin.entity.display_pos_y,
    //         is_deleted: coin.entity.is_deleted,
    //         destination_y: coin.entity.destination_y,
    //         age_since_spawned: coin.entity.age_since_spawned,
    //         age_since_reached_destination: coin.entity.age_since_reached_destination,
    //         content: CoinContent::Diamond,
    //     };
    //     coin.write_entity(&proc);
    // }
}
