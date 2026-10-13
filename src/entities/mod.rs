//! The game's objects: plants, zombies and everything else in a level.
//!
//! [`Board::current`] reads the level being played, and its `Entity<Board>` reads what's on
//! the lawn. Each object is an [`Entity`], which can be changed and saved back into the game.
//!
//! Each file starts with a type as the rest of the code uses it, like [`Plant`]. At the
//! bottom are the fields of the game's class that it's made from (a `class_layout!`, see
//! [`crate::il2cpp`]), and the [`GameObject`] impl that converts between the two. To add a
//! field:
//!
//! 1. Find it in the class in Il2CppDumper's `dump.cs`, and add it to the `class_layout!` with
//!    its C# name and the Rust type to read it as.
//! 2. Add it to the type, in whatever form is easiest to use, and convert it in `from_fields`
//!    and `update_fields`.
//! 3. Give it different values in the two objects of the file's `round_trips` test, which
//!    checks that `update_fields` writes back everything `from_fields` reads.
//!
//! The enums have the game's values. Where a variant is named differently from the game's
//! enum, a comment after it has the game's name, to find it in `dump.cs`.

mod board;
mod challenge;
mod coin;
mod common;
mod data_array;
mod entity;
mod grid_item;
mod id;
mod lawn_mower;
mod level;
mod plant;
mod potted_plant;
mod projectile;
mod seed_packet;
mod zombie;

use crate::il2cpp::TypeInfo;

pub use board::{Background, Board, ROOT_MODEL, Sun};
pub use challenge::{BeghouledUpgrade, Challenge, ChallengeState};
pub use coin::{Coin, CoinKind, CoinMotion};
pub use common::{Cell, Health, Position, Ticks};
pub use data_array::BoardObject;
pub use entity::{Entity, GameObject};
pub use grid_item::{
    GridItem, GridItemKind, PortalShape, RakeState, SquirrelState, StinkyState, Vase, VaseContent,
    VaseKind, ZenTool,
};
pub use id::Id;
pub use lawn_mower::{LawnMower, LawnMowerState, LawnMowerType};
pub use level::{GameArea, GameMode, LevelCategory, LevelInfo, ReloadedGameMode};
pub use plant::{BungeeGrab, Plant, PlantState, PlantType};
pub use potted_plant::{GardenType, PottedPlant, PottedPlantAge, PottedPlantNeed};
pub use projectile::{Projectile, ProjectileMotion, ProjectileType};
pub use seed_packet::{PacketState, Seed, SeedPacket};
pub use zombie::{Armor, HeadwearType, ShieldType, Zombie, ZombiePhase, ZombieType};

/// Every `TypeInfo` global that reading the entities starts from. Their addresses change with
/// every game update, and `re-tools check` checks them against Il2CppDumper's `script.json`.
pub const TYPE_INFOS: &[TypeInfo] = &[ROOT_MODEL];
