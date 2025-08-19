#![allow(dead_code)]

use std::fmt::Debug;

enum ArmorType {
    None,
    Cone,
    Bucket,
}

impl From<ArmorType> for u32 {
    fn from(val: ArmorType) -> Self {
        match val {
            ArmorType::None => 0,
            ArmorType::Cone => 1,
            ArmorType::Bucket => 2,
        }
    }
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
    pub zombie_type: u32, //ZombieType,
    _pad4: [u8; 4],
    pub zombie_pos_x: f32,
    pub zombie_pos_y: f32,
    _pad5: [u8; 144],
    pub cone_type: u32,
    pub health: i32,
    _pad6: [u8; 4],
    pub cone_hp: u32,
    _pad7: [u8; 24],
    pub is_dead: u32,
    _pad8: [u8; 120],
}

impl Debug for Zombie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_pos_x = self.display_pos_x;
        let display_pos_y = self.display_pos_y;
        let row = self.row;
        let zombie_type = self.zombie_type;
        let zombie_pos_x = self.zombie_pos_x;
        let zombie_pos_y = self.zombie_pos_y;
        let cone_type = self.cone_type;
        let health = self.health;
        let cone_hp = self.cone_hp;
        let is_dead = self.is_dead;

        f.debug_struct("Zombie")
            .field("display_pos_x", &display_pos_x)
            .field("display_pos_y", &display_pos_y)
            .field("row", &row)
            .field("zombie_type", &zombie_type)
            .field("zombie_pos_x", &zombie_pos_x)
            .field("zombie_pos_y", &zombie_pos_y)
            .field("cone_type", &cone_type)
            .field("health", &health)
            .field("cone_hp", &cone_hp)
            .field("is_dead", &is_dead)
            .finish()
    }
}
