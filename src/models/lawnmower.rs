use std::{fmt::Debug, mem::transmute};

#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Lawnmower {
    _pad1: [u8; 8],
    pub display_pos_x: f32,
    pub display_pos_y: f32,
    _pad2: [u8; 4],
    pub row: u32,
    _pad3: [u8; 20],
    pub lawnmower_mode: LawnmowerMode,
    pub is_deleted: bool,
    _pad4: [u8; 3],
    pub lawnmower_type: LawnmowerType,
    _pad5: [u8; 16],
}

impl Debug for Lawnmower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lawnmower")
            .field("display_pos_x", &{ self.display_pos_x })
            .field("display_pos_y", &{ self.display_pos_y })
            .field("row", &{ self.row })
            .field("lawnmower_mode", &{ self.lawnmower_mode })
            .field("is_deleted", &self.is_deleted)
            .field("lawnmower_type", &{ self.lawnmower_type })
            .finish()
    }
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum LawnmowerType {
    Normal = 0,
    PoolCleaner = 1,
    RoofCleaner = 2,
    Unidentifiable = 3,
}

impl Debug for LawnmowerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_value = unsafe { transmute::<&Self, &u32>(self) };
        if !matches!(raw_value, 0..=3) {
            return write!(f, "{raw_value}");
        }

        match self {
            Self::Normal => write!(f, "Normal"),
            Self::PoolCleaner => write!(f, "PoolCleaner"),
            Self::RoofCleaner => write!(f, "RoofCleaner"),
            Self::Unidentifiable => write!(f, "Unidentifiable"),
        }
    }
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum LawnmowerMode {
    Resetting = 0,
    Normal = 1,
    Running = 2,
}

impl Debug for LawnmowerMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_value = unsafe { transmute::<&Self, &u32>(self) };
        if !matches!(raw_value, 0..=2) {
            return write!(f, "{raw_value}");
        }

        match self {
            Self::Resetting => write!(f, "Resetting"),
            Self::Normal => write!(f, "Normal"),
            Self::Running => write!(f, "Running"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_struct_size() {
        assert_eq!(size_of::<Lawnmower>(), 72);
    }
}
