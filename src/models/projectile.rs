use std::fmt::Debug;

use super::ProjectileType;

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Projectile {
    _pad1: [u8; 8],
    pub display_pos_x: u32,
    pub display_pos_y: u32,
    _pad2: [u8; 32],
    pub pos_x: f32,
    pub pos_y_1: f32,
    _pad3: [u8; 20],
    pub pos_y_2: f32,
    pub is_deleted: bool,
    _pad4: [u8; 11],
    pub projectile_type: u32,
    _pad5: [u8; 52],
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            _pad1: Default::default(),
            display_pos_x: Default::default(),
            display_pos_y: Default::default(),
            _pad2: Default::default(),
            pos_x: Default::default(),
            pos_y_1: Default::default(),
            _pad3: Default::default(),
            pos_y_2: Default::default(),
            is_deleted: Default::default(),
            _pad4: Default::default(),
            projectile_type: Default::default(),
            _pad5: [0; _],
        }
    }
}

impl Debug for Projectile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Projectile")
            .field("display_pos_x", &{ self.display_pos_x })
            .field("display_pos_y", &{ self.display_pos_y })
            .field("pos_x", &{ self.pos_x })
            .field("pos_y_1", &{ self.pos_y_1 })
            .field("pos_y_2", &{ self.pos_y_2 })
            .field("is_deleted", &{ self.is_deleted })
            .field("projectile_type", &{ self.projectile_type })
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_struct_size() {
        assert_eq!(size_of::<Projectile>(), 148);
    }
}
