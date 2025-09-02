use super::{ArmorType, ZombieType};

use std::fmt::Debug;

#[derive(Debug)]
pub struct Zombies {
    pub capacity: u32,
    pub next_index: u32,
    pub count: u32,
    pub zombies: Vec<Zombie>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Zombie {
    pub display_pos_x: u32,
    pub display_pos_y: u32,
    pub row: u32,
    pub zombie_type: ZombieType,
    pub pos_x: f32,
    pub pos_y: f32,
    pub freeze_timer: u32,
    /// Is the zombie attacking other zombies
    pub is_hypnotized: bool,
    pub armor_type: ArmorType,
    pub health: i32,
    pub original_health: i32,
    pub armor_hp: u32,
    pub original_armor_hp: u32,
    pub is_dead: bool,
}
