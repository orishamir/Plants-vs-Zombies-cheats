use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Debug;

use crate::entities::{PlantType, ZombieType};

#[derive(Debug)]
pub struct Griditems {
    pub capacity: u32,
    pub next_index: u32,
    pub count: u32,
    pub griditems: Vec<Griditem>,
}

#[derive(Debug)]
pub struct Griditem {
    pub addr: usize,

    pub is_deleted: bool,
    pub content: GriditemContent,
}

#[derive(Debug, Clone, Copy)]
pub enum GriditemContent {
    Portal,
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
    /// WateringCan / BugSpray / MusicPlayer / Chocolate
    ZenGardenItem,
    Snail {
        pos_x: f32,
        pos_y: f32,
        destination_x: f32,
        destination_y: f32,
    },
    EatableBrain {
        pos_x: f32,
        pos_y: f32,
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

#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum GriditemContentType {
    GraveBuster = 1,
    DoomShroomCrater = 2,
    Portal = 5,
    /// The ones you drop the zombies in "Zombiquarium"
    EatableBrain = 6,
    Vase = 7,
    // WateringCan / BugSpray / MusicPlayer / Chocolate
    ZenGardenItem = 9,
    Snail = 10,
    Rake = 11,
    // The brain in the reverse-zombie puzzle thingy
    Brain = 12,
}
