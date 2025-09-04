use super::{LawnmowerMode, LawnmowerType};

use std::fmt::Debug;

#[derive(Debug)]
pub struct Lawnmowers {
    pub capacity: u32,
    pub next_index: u32,
    pub count: u32,
    pub lawnmowers: Vec<Lawnmower>,
}

#[derive(Debug)]
pub struct Lawnmower {
    pub addr: usize,
    pub display_pos_x: f32,
    pub display_pos_y: f32,
    pub row: u32,
    pub mode: LawnmowerMode,
    pub is_deleted: bool,
    pub lawnmower_type: LawnmowerType,
}
