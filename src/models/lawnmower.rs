use std::{fmt::Debug, mem::transmute};

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

#[allow(dead_code)]
#[repr(C, packed)]
pub struct Lawnmower {
    display_pos_x: f32,
    display_pos_y: f32,
    row: u32,
    lawnmower_mode: LawnmowerMode,
    is_deleted: u8,
    lawnmower_type: LawnmowerType,
}
