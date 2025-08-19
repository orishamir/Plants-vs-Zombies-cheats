#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
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
}

impl Into<u32> for ProjectileType {
    fn into(self) -> u32 {
        match self {
            Self::Pea => 0,
            Self::FrozenPea => 1,
            Self::Cabbage => 2,
            Self::Watermelon => 3,
            Self::Puff => 4,
            Self::WinterMelon => 5,
            Self::FlamingPea => 6,
            Self::Star => 7,
            Self::Cacti => 8,
            Self::Basketball => 9,
            Self::Corn => 10,
            Self::GiantCorn => 11,
            Self::Butterstick => 12,
        }
    }
}
