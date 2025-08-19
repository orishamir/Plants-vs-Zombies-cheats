use std::fmt::Debug;

use super::plant_type::PlantType;

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Plant {
    _pad1: [u8; 8],
    pub display_pos_x: u32,
    pub display_pos_y: u32,
    _pad2: [u8; 12],
    pub row: u32,
    _pad3: [u8; 4],
    pub plant_type: u32, //PlantType
    pub column: u32,
    _pad4: [u8; 16],
    pub plant_state: u32,
    pub health: u32,
    pub max_health: u32,
    _pad5: [u8; 249],
    pub is_deleted: u8,
    _pad6: [u8; 3],
    pub is_considered_shoveling: u8,
    _pad7: [u8; 6],
}

impl Debug for Plant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plant")
            .field("display_pos_x", &{ self.display_pos_x })
            .field("display_pos_y", &{ self.display_pos_y })
            .field("row", &{ self.row })
            .field("plant_type", &{ self.plant_type })
            .field("column", &{ self.column })
            .field("plant_state", &{ self.plant_state })
            .field("health", &{ self.health })
            .field("max_health", &{ self.max_health })
            .field("is_deleted", &self.is_deleted)
            .field("is_considered_shoveling", &self.is_considered_shoveling)
            .finish()
    }
}
