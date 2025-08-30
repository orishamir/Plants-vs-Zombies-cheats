use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::PlantType;

use std::fmt::Debug;

/// Game memory calls this a Coin, but it is any pickable item
#[allow(dead_code)]
#[derive(Debug)]
pub struct Coin {
    pub display_pos_x: f32,
    pub display_pos_y: f32,
    pub is_deleted: bool,
    pub destination_y: u32,
    pub age_since_spawned: u32,
    pub age_since_reached_destination: u32,
    pub content: CoinContent,
}

#[derive(Debug)]
pub enum CoinContent {
    Silver,
    Gold,
    Diamond,
    Sun,
    MiniSun,
    DroppedCard { plant_type: PlantType },
    GiantBagOfCash,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum CoinType {
    Silver = 1,
    Gold = 2,
    Diamond = 3,
    Sun = 4,
    MiniSun = 5,
    DroppedCard = 16,
    GiantBagOfCash = 18,
}
