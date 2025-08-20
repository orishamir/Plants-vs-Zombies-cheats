use std::{fmt::Debug, mem::transmute};

use egui::Grid;

use crate::models::{PlantType, ZombieType};

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Griditem {
    _pad1: [u8; 8],
    pub griditem_type: GriditemType,
    pub vase_type: VaseType,
    pub column: u32,
    pub row: u32,
    pub timer_until_dead: u32,
    _pad3: [u8; 4],
    pub is_deleted: u32,
    _pad4: [u8; 24],
    pub zombie_type: ZombieType,
    pub plant_type: PlantType,
    _pad5: [u8; 4],
    pub is_highlighted: bool,
    _pad6: [u8; 3],
    pub opacity: u32,
    _pad7: [u8; 156],
}

impl Debug for Griditem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Griditem")
            .field("griditem_type", &{ self.griditem_type })
            .field("vase_type", &{ self.vase_type })
            .field("column", &{ self.column })
            .field("row", &{ self.row })
            .field("timer_until_dead", &{ self.timer_until_dead })
            .field("is_deleted", &{ self.is_deleted })
            .field("zombie_type", &{ self.zombie_type })
            .field("plant_type", &{ self.plant_type })
            .field("is_highlighted", &{ self.is_highlighted })
            .field("opacity", &{ self.opacity })
            .finish()
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum GriditemType {
    Grave = 1,
    DoomShroomCrater = 2,
    Vase = 7,
    Snail = 10,
    Rake = 11,
}

impl Debug for GriditemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_value = unsafe { transmute::<&Self, &u32>(self) };
        if !matches!(raw_value, 1 | 2 | 7 | 10 | 11) {
            return write!(f, "{raw_value}");
        }
        match self {
            Self::Grave => write!(f, "Grave"),
            Self::DoomShroomCrater => write!(f, "DoomShroomCrater"),
            Self::Vase => write!(f, "Vase"),
            Self::Snail => write!(f, "Snail"),
            Self::Rake => write!(f, "Rake"),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum VaseType {
    /// Normal vase
    Hidden = 3,
    /// The green vase
    ConfirmedPlant = 4,
    /// The zombie vase
    ConfirmedZombie = 5,
}

impl Debug for VaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_value = unsafe { transmute::<&Self, &u32>(self) };
        if !matches!(raw_value, 3..=5) {
            return write!(f, "{raw_value}");
        }

        match self {
            Self::Hidden => write!(f, "Hidden"),
            Self::ConfirmedPlant => write!(f, "ConfirmedPlant"),
            Self::ConfirmedZombie => write!(f, "ConfirmedZombie"),
        }
    }
}
