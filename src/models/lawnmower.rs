use super::{LawnmowerMode, LawnmowerType};

use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Lawnmower {
    pub display_pos_x: f32,
    pub display_pos_y: f32,
    pub row: u32,
    pub lawnmower_mode: LawnmowerMode,
    pub is_deleted: bool,
    pub lawnmower_type: LawnmowerType,
}
