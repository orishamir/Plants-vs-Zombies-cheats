use std::{fmt::Debug, mem::transmute};

#[allow(dead_code)]
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ProjectileType {
    Pea = 0,
    FrozenPea = 1,
    Cabbage = 2,
    Watermelon = 3,
    Puff = 4,
    WinterMelon = 5,
    FlamingPea = 6,
    Star = 7,
    Cacti = 8,
    Basketball = 9,
    Corn = 10,
    GiantCorn = 11,
    Butterstick = 12,
}

impl Debug for ProjectileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_value = unsafe { transmute::<&Self, &u32>(self) };
        if !matches!(raw_value, 0..=12) {
            return write!(f, "{raw_value}");
        }

        match self {
            Self::Pea => write!(f, "Pea"),
            Self::FrozenPea => write!(f, "FrozenPea"),
            Self::Cabbage => write!(f, "Cabbage"),
            Self::Watermelon => write!(f, "Watermelon"),
            Self::Puff => write!(f, "Puff"),
            Self::WinterMelon => write!(f, "WinterMelon"),
            Self::FlamingPea => write!(f, "FlamingPea"),
            Self::Star => write!(f, "Star"),
            Self::Cacti => write!(f, "Cacti"),
            Self::Basketball => write!(f, "Basketball"),
            Self::Corn => write!(f, "Corn"),
            Self::GiantCorn => write!(f, "GiantCorn"),
            Self::Butterstick => write!(f, "Butterstick"),
        }
    }
}
