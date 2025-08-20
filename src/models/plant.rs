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
    pub plant_type: PlantType,
    pub column: u32,
    _pad4: [u8; 16],
    pub plant_state: u32,
    pub health: u32,
    pub max_health: u32,
    _pad5: [u8; 12],
    /// Depends on the plant.
    /// - Chomper: The time left to digest the zombie.
    /// - Cob cannon: The time left until cob is ready to shoot.
    pub plant_timer: u32,
    _pad6: [u8; 233],
    pub is_deleted: bool,
    _pad7: [u8; 3],
    pub is_considered_shoveling: bool,
    _pad8: [u8; 6],
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
            .field("plant_timer", &{ self.plant_timer })
            .field("is_deleted", &self.is_deleted)
            .field("is_considered_shoveling", &self.is_considered_shoveling)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_struct_size() {
        assert_eq!(size_of::<Plant>(), 332);
    }
}
