#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum LawnmowerType {
    Normal = 0,
    PoolCleaner = 1,
    RoofCleaner = 2,
    Unidentifiable = 3,
}

#[allow(dead_code)]
#[repr(u32)]
pub enum LawnmowerMode {
    Resetting = 0,
    Normal = 1,
    Running = 2,
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
