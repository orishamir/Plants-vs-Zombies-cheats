#![allow(dead_code)]

use std::fmt::Debug;

use super::zombie_type::ZombieType;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum ArmorType {
    None = 0,
    Cone = 1,
    Bucket = 2,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Zombie {
    _pad1: [u8; 8],
    pub display_pos_x: u32,
    pub display_pos_y: u32,
    _pad2: [u8; 12],
    pub row: u32,
    _pad3: [u8; 4],
    pub zombie_type: ZombieType,
    _pad4: [u8; 4],
    pub zombie_pos_x: f32,
    pub zombie_pos_y: f32,
    _pad5: [u8; 144],
    pub armor_type: ArmorType,
    pub health: i32,
    _pad6: [u8; 4],
    pub armor_hp: u32,
    _pad7: [u8; 24],
    pub is_dead: u32,
    _pad8: [u8; 120],
}

impl Debug for Zombie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Zombie")
            .field("display_pos_x", &{ self.display_pos_x })
            .field("display_pos_y", &{ self.display_pos_y })
            .field("row", &{ self.row })
            .field("zombie_type", &{ self.zombie_type })
            .field("zombie_pos_x", &{ self.zombie_pos_x })
            .field("zombie_pos_y", &{ self.zombie_pos_y })
            .field("armor_type", &{ self.armor_type })
            .field("health", &{ self.health })
            .field("armor_hp", &{ self.armor_hp })
            .field("is_dead", &{ self.is_dead })
            .finish()
    }
}
