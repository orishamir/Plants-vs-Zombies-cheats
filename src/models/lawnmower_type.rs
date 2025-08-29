use num_enum::{FromPrimitive, IntoPrimitive};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum LawnmowerType {
    Normal = 0,
    PoolCleaner = 1,
    RoofCleaner = 2,
    Unidentifiable = 3,
    #[num_enum(catch_all)]
    Unknown(u32),
}

impl Default for LawnmowerType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum LawnmowerMode {
    Resetting = 0,
    Normal = 1,
    Running = 2,
    #[num_enum(catch_all)]
    Unknown(u32),
}

impl Default for LawnmowerMode {
    fn default() -> Self {
        Self::Unknown(0)
    }
}
