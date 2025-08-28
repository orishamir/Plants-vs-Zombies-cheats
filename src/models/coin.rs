use super::{CoinType, PlantType};

use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::Debug;

/// Game memory calls this a Coin, but it is any pickable item
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Coin {
    pub display_pos_x: f32,
    pub display_pos_y: f32,
    pub is_deleted: bool,
    pub destination_y: u32,
    pub age_since_spawned: u32,
    pub age_since_reached_destination: u32,
    pub coin_type: CoinType,
    /// If CoinType is DroppedCard, this is the plant.
    /// For example winning a new card or a broken vase's drop.
    pub plant_type: PlantType,
}
