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

impl From<ProjectileType> for u32 {
    fn from(val: ProjectileType) -> Self {
        match val {
            ProjectileType::Pea => 0,
            ProjectileType::FrozenPea => 1,
            ProjectileType::Cabbage => 2,
            ProjectileType::Watermelon => 3,
            ProjectileType::Puff => 4,
            ProjectileType::WinterMelon => 5,
            ProjectileType::FlamingPea => 6,
            ProjectileType::Star => 7,
            ProjectileType::Cacti => 8,
            ProjectileType::Basketball => 9,
            ProjectileType::Corn => 10,
            ProjectileType::GiantCorn => 11,
            ProjectileType::Butterstick => 12,
        }
    }
}
