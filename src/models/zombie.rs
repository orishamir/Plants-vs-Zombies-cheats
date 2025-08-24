#![allow(dead_code)]

use std::{fmt::Debug, mem::transmute};

use super::ZombieType;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ArmorType {
    None = 0,
    Cone = 1,
    Bucket = 2,
    Football = 3,
}

impl Debug for ArmorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_value = unsafe { transmute::<&Self, &u32>(self) };
        if !matches!(raw_value, 0..=2) {
            return write!(f, "{raw_value}");
        }

        match self {
            Self::None => write!(f, "None"),
            Self::Cone => write!(f, "Cone"),
            Self::Bucket => write!(f, "Bucket"),
            Self::Football => write!(f, "Football"),
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Zombie {
    _pad1: [u8; 8],
    pub display_pos_x: u32,
    pub display_pos_y: u32,
    _pad2: [u8; 12],
    pub row: u32,
    _pad3: [u8; 4],
    pub zombie_type: ZombieType,
    _pad4: [u8; 4],
    pub zombie_pos_x: f32,
    pub zombie_pos_y: f32,
    _pad5: [u8; 144],
    pub armor_type: ArmorType,
    pub health: i32,
    _pad6: [u8; 4],
    pub armor_hp: u32,
    _pad7: [u8; 24],
    pub is_dead: bool,
    _pad8: [u8; 123],
}

impl Default for Zombie {
    fn default() -> Self {
        Self {
            _pad1: Default::default(),
            display_pos_x: Default::default(),
            display_pos_y: Default::default(),
            _pad2: Default::default(),
            row: Default::default(),
            _pad3: Default::default(),
            zombie_type: ZombieType::Zombie,
            _pad4: Default::default(),
            zombie_pos_x: Default::default(),
            zombie_pos_y: Default::default(),
            _pad5: [0; _],
            armor_type: ArmorType::None,
            health: Default::default(),
            _pad6: Default::default(),
            armor_hp: Default::default(),
            _pad7: Default::default(),
            is_dead: Default::default(),
            _pad8: [0; _],
        }
    }
}

impl Debug for Zombie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Zombie")
            .field("display_pos_x", &{ self.display_pos_x })
            .field("display_pos_y", &{ self.display_pos_y })
            .field("row", &{ self.row })
            .field("zombie_type", &{ self.zombie_type })
            .field("zombie_pos_x", &{ self.zombie_pos_x })
            .field("zombie_pos_y", &{ self.zombie_pos_y })
            .field("armor_type", &{ self.armor_type })
            .field("health", &{ self.health })
            .field("armor_hp", &{ self.armor_hp })
            .field("is_dead", &{ self.is_dead })
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_struct_size() {
        assert_eq!(size_of::<Zombie>(), 360);
    }
}
