//! Prints the level being played: the board and everything on it.

use std::fmt::Debug;

use pvz_sdk::Popcapgame;
use pvz_sdk::entities::Board;
use pvz_sdk::il2cpp::ObjectError;

fn main() -> anyhow::Result<()> {
    let game = Popcapgame::init()?;
    let Some(board) = Board::current(&game)? else {
        println!("Not in a level.");
        return Ok(());
    };

    println!("{board:#?}");
    print_all("Level", board.level_info(&game).map(Vec::from_iter));
    match board.challenge(&game) {
        Ok(Some(challenge)) => {
            print_all("Challenge", Ok(vec![&challenge]));
            print_all("Beghouled craters", challenge.beghouled_craters(&game));
            print_all("Beghouled upgrades", challenge.beghouled_upgrades(&game));
        }
        other => print_all("Challenge", other.map(Vec::from_iter)),
    }
    print_all("Sun", board.sun(&game).map(|sun| vec![sun]));
    print_all("Seed packets", board.seed_packets(&game));
    print_all("Plants", board.plants(&game));
    print_all(
        "Potted plants",
        board.plants(&game).and_then(|plants| {
            plants
                .iter()
                .filter_map(|plant| plant.potted_plant(&game).transpose())
                .collect()
        }),
    );
    print_all("Zombies", board.zombies(&game));
    print_all("Projectiles", board.projectiles(&game));
    print_all("Coins", board.coins(&game));
    print_all("Lawn mowers", board.lawn_mowers(&game));
    print_all("Grid items", board.grid_items(&game));
    Ok(())
}

/// Prints a list of entities, or why it couldn't be read.
fn print_all<T: Debug>(title: &str, entities: Result<Vec<T>, ObjectError>) {
    println!("----- {title}");
    match entities {
        Ok(entities) => entities.iter().for_each(|entity| println!("{entity:#?}")),
        Err(error) => println!("Couldn't read them: {error}"),
    }
}
