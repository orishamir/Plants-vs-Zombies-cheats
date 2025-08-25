#[allow(dead_code)]
#[derive(Debug)]
pub enum LawnmowerType {
    Normal,
    PoolCleaner,
    RoofCleaner,
    Unidentifiable,
    Unknown(u32),
}

impl Default for LawnmowerType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl From<u32> for LawnmowerType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::PoolCleaner,
            2 => Self::RoofCleaner,
            3 => Self::Unidentifiable,
            val => Self::Unknown(val),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum LawnmowerMode {
    Resetting,
    Normal,
    Running,
    Unknown(u32),
}

impl Default for LawnmowerMode {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl From<u32> for LawnmowerMode {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Resetting,
            1 => Self::Normal,
            2 => Self::Running,
            val => Self::Unknown(val),
        }
    }
}
