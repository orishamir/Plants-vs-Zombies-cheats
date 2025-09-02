use num_enum::{IntoPrimitive, TryFromPrimitive};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum LawnmowerType {
    Normal = 0,
    PoolCleaner = 1,
    RoofCleaner = 2,
    Unidentifiable = 3,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum LawnmowerMode {
    Resetting = 0,
    Normal = 1,
    Running = 2,
}
