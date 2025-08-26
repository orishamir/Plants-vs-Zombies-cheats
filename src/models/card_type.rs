use super::{PlantType, ZombieType};

#[derive(Debug)]
pub enum CardType {
    Plant(PlantType),
    Zombie(ZombieType),

    ShufflePlants,
    Crater,
    Sun,
    Diamond,
    SnorkelZombie,
    GoldTrophie,

    Unknown(u32),
}

impl Default for CardType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl From<u32> for CardType {
    fn from(value: u32) -> Self {
        match value {
            plant_val if matches!(plant_val, 0..=47 | 52) => CardType::Plant(plant_val.into()),

            54 => CardType::ShufflePlants,
            55 => CardType::Crater,
            56 => CardType::Sun,
            57 => CardType::Diamond,
            58 => CardType::SnorkelZombie,
            59 => CardType::GoldTrophie,

            60 => Self::Zombie(ZombieType::Zombie),
            61 => Self::Zombie(ZombieType::ConeheadZombie),
            62 => Self::Zombie(ZombieType::PoleVaultingZombie),
            63 => Self::Zombie(ZombieType::BucketheadZombie),
            64 => Self::Zombie(ZombieType::LadderZombie),
            65 => Self::Zombie(ZombieType::DiggerZombie),
            66 => Self::Zombie(ZombieType::BungeeZombie),
            67 => Self::Zombie(ZombieType::FootballZombie),
            68 => Self::Zombie(ZombieType::BalloonZombie),
            69 => Self::Zombie(ZombieType::ScreenDoorZombie),
            70 => Self::Zombie(ZombieType::Zomboni),
            71 => Self::Zombie(ZombieType::PogoZombie),
            72 => Self::Zombie(ZombieType::DancingZombie),
            73 => Self::Zombie(ZombieType::GigaGargantuar),
            74 => Self::Zombie(ZombieType::Imp),

            val => Self::Unknown(val),
        }
    }
}
