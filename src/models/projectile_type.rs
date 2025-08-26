use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ProjectileType {
    Pea,
    FrozenPea,
    Cabbage,
    Watermelon,
    Puff,
    WinterMelon,
    FlamingPea,
    Star,
    Cacti,
    Basketball,
    Corn,
    GiantCorn,
    Butterstick,
    Unknown(u32),
}

impl Default for ProjectileType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl From<u32> for ProjectileType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Pea,
            1 => Self::FrozenPea,
            2 => Self::Cabbage,
            3 => Self::Watermelon,
            4 => Self::Puff,
            5 => Self::WinterMelon,
            6 => Self::FlamingPea,
            7 => Self::Star,
            8 => Self::Cacti,
            9 => Self::Basketball,
            10 => Self::Corn,
            11 => Self::GiantCorn,
            12 => Self::Butterstick,
            val => Self::Unknown(val),
        }
    }
}
