use pvz_sdk::{
    Popcapgame,
    entities::{Coins, Griditems, Lawnmowers, Plants, Projectiles, Slots, Zombies},
    readers::ReadableEntity,
};

fn main() {
    let game = Popcapgame::init().unwrap();

    let slots = Slots::read(&game);
    let coins = Coins::read(&game);
    let lawnmowers = Lawnmowers::read(&game);
    let griditems = Griditems::read(&game);
    let projectiles = Projectiles::read(&game);
    let plants = Plants::read(&game);
    let zombies = Zombies::read(&game);

    println!("{coins:#?}");
    println!("-----------------------------");
    println!("{lawnmowers:#?}");
    println!("-----------------------------");
    println!("{griditems:#?}");
    println!("-----------------------------");
    println!("{projectiles:#?}");
    println!("-----------------------------");
    println!("{plants:#?}");
    println!("-----------------------------");
    println!("{zombies:#?}");
}
