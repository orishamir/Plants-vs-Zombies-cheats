use super::ProjectileType;

use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Projectile {
    pub display_pos_x: u32,
    pub display_pos_y: u32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub collision_y: f32,
    pub is_deleted: bool,
    pub projectile_type: ProjectileType,
}
