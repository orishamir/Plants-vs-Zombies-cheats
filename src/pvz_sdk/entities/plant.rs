use super::PlantType;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Plants {
    pub capacity: u32,
    pub next_index: u32,
    pub count: u32,
    pub plants: Vec<Plant>,
}

#[derive(Debug)]
pub struct Plant {
    pub addr: usize,

    pub display_pos_x: u32,
    pub display_pos_y: u32,
    pub row: u32,
    pub plant_type: PlantType,
    pub column: u32,
    /// A field which value depends on the plant type.
    /// If its a bowling wall-nut, determines the direction that the wallnut bowls
    pub plant_state: u32,
    pub health: u32,
    pub original_health: u32,
    /// Depends on the plant.
    /// - Chomper: The time left to digest the zombie.
    /// - Cob cannon: The time left until cob is ready to shoot.
    pub plant_timer: u32,
    /// In bowling wallnuts, how many zombies the wallnut hit
    pub hit_counter: u32,
    pub is_deleted: bool,
    pub is_asleep: bool,
    pub is_considered_shoveling: bool,
}
