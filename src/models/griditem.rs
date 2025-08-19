use std::fmt::Debug;

use egui::Grid;

use crate::models::{PlantType, ZombieType};

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Griditem {
    _pad1: [u8; 8],
    pub griditem_type: GriditemType,
    _pad2: [u8; 4],
    pub column: u32,
    pub row: u32,
    pub timer_until_dead: u32,
    _pad3: [u8; 4],
    pub is_deleted: u32,
    _pad4: [u8; 24],
    pub zombie_type: ZombieType,
    pub plant_type: PlantType,
    _pad5: [u8; 4],
    pub is_highlighted: u32,
    pub is_seethrough: u32,
    _pad6: [u8; 156],
}

impl Debug for Griditem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Griditem")
            .field("griditem_type", &{ self.griditem_type })
            .field("column", &{ self.column })
            .field("row", &{ self.row })
            .field("timer_until_dead", &{ self.timer_until_dead })
            .field("is_deleted", &{ self.is_deleted })
            .field("zombie_type", &{ self.zombie_type })
            .field("plant_type", &{ self.plant_type })
            .field("is_highlighted", &{ self.is_highlighted })
            .field("is_seethrough", &{ self.is_seethrough })
            .finish()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum GriditemType {
    Grave = 1,
    DoomShroomCrater = 2,
    Vase = 7,
    Snail = 10,
    Rake = 11,
}
