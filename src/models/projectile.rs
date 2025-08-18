use crate::models::projectile_type::ProjectileType;

#[allow(dead_code)]
pub struct Projectile {
    display_pos_x: u32,
    display_pos_y: u32,
    pos_x: f32,
    pos_y: f32,
    projectile_type: ProjectileType,
}
