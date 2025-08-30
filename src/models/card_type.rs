use super::{PlantType, ZombieType};

#[derive(Debug, Clone, Copy)]
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

impl From<CardType> for u32 {
    fn from(val: CardType) -> Self {
        match val {
            CardType::Plant(plant_type) => plant_type.into(),
            CardType::Zombie(ZombieType::Zombie) => 60,
            CardType::Zombie(ZombieType::ConeheadZombie) => 61,
            CardType::Zombie(ZombieType::PoleVaultingZombie) => 62,
            CardType::Zombie(ZombieType::BucketheadZombie) => 63,
            CardType::Zombie(ZombieType::LadderZombie) => 64,
            CardType::Zombie(ZombieType::DiggerZombie) => 65,
            CardType::Zombie(ZombieType::BungeeZombie) => 66,
            CardType::Zombie(ZombieType::FootballZombie) => 67,
            CardType::Zombie(ZombieType::BalloonZombie) => 68,
            CardType::Zombie(ZombieType::ScreenDoorZombie) => 69,
            CardType::Zombie(ZombieType::Zomboni) => 70,
            CardType::Zombie(ZombieType::PogoZombie) => 71,
            CardType::Zombie(ZombieType::DancingZombie) => 72,
            CardType::Zombie(ZombieType::GigaGargantuar) => 73,
            CardType::Zombie(ZombieType::Imp) => 74,
            CardType::ShufflePlants => 54,
            CardType::Crater => 55,
            CardType::Sun => 56,
            CardType::Diamond => 57,
            CardType::SnorkelZombie => 58,
            CardType::GoldTrophie => 59,
            CardType::Zombie(unknown_zombie) => {
                unreachable!("Zombie type can't be used for a Card: {unknown_zombie:?}")
            }
            CardType::Unknown(val) => val,
        }
    }
}
