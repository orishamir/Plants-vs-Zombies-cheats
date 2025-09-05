use pvz_sdk::{
    Popcapgame,
    entities::{Coins, Griditems, Lawnmowers, Plants, Projectiles, Slots, Zombies},
    readers::ReadableEntity,
};

fn main() {
    let game = Popcapgame::init().unwrap();

    let slots = Slots::read(&game).unwrap();
    let coins = Coins::read(&game).unwrap();
    let lawnmowers = Lawnmowers::read(&game).unwrap();
    let griditems = Griditems::read(&game).unwrap();
    let projectiles = Projectiles::read(&game).unwrap();
    let plants = Plants::read(&game).unwrap();
    let zombies = Zombies::read(&game).unwrap();

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
