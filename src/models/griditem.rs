use super::{GriditemType, PlantType, VaseType, ZombieType};

use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Griditem {
    pub griditem_type: GriditemType,
    pub vase_type: VaseType,
    pub column: u32,
    pub row: u32,
    pub timer_until_dead: u32,
    pub is_deleted: bool,
    pub zombie_type: ZombieType,
    pub plant_type: PlantType,
    pub is_highlighted: bool,
    pub opacity: u32,
}
