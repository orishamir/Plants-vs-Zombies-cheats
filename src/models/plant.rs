use super::PlantType;
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Plant {
    pub display_pos_x: u32,
    pub display_pos_y: u32,
    pub row: u32,
    pub plant_type: PlantType,
    pub column: u32,
    pub plant_state: u32,
    pub health: u32,
    pub original_health: u32,
    /// Depends on the plant.
    /// - Chomper: The time left to digest the zombie.
    /// - Cob cannon: The time left until cob is ready to shoot.
    pub plant_timer: u32,
    pub is_deleted: bool,
    pub is_considered_shoveling: bool,
}
