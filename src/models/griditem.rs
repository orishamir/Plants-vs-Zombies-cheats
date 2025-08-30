use crate::models::{PlantType, ZombieType};

use std::fmt::Debug;

#[derive(Debug)]
pub struct Griditem {
    pub is_deleted: bool,
    pub content: GriditemContent,
}

#[derive(Debug, Clone, Copy)]
pub enum GriditemContent {
    Vase {
        column: u32,
        row: u32,
        is_highlighted: bool,
        opacity: u32,
        vase_kind: VaseKind,
        vase_content: VaseContent,
    },
    GraveBuster,
    DoomShroomCrater,
    ZenGardenItem,
    Snail {
        pos_x: f32,
        pos_y: f32,
        destination_x: f32,
        destination_y: f32,
    },
    Rake,
    Brain,
}

#[derive(Debug, Clone, Copy)]
/// See [Kinds of vases](https://plantsvszombies.fandom.com/wiki/Vasebreaker?file=Scary_Pot.png)
pub enum VaseKind {
    /// Normal vase
    Mistery,
    /// The green vase
    Plant,
    /// The zombie vase, not used in-game at all.
    Zombie,
}

#[derive(Debug, Clone, Copy)]
pub enum VaseContent {
    Zombie { zombie_type: ZombieType },
    Plant { plant_type: PlantType },
    Sun { sun_count: u32 },
}
