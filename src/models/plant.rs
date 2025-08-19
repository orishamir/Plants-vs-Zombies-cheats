use super::plant_type::PlantType;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Plant {
    display_pos_x: u32,
    display_pos_y: u32,
    row: u32,
    plant_type: PlantType,
    column: u32,
    health: u32,
    max_health: u32,
    is_deleted: u8,
    is_considered_shoveling: u8,
}
