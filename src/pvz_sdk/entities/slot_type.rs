use anyhow::bail;

use super::{PlantType, ZombieType};

#[derive(Debug, Clone, Copy)]
pub enum SlotType {
    Plant(PlantType),
    Zombie(ZombieType),

    ShufflePlants,
    Crater,
    Sun,
    Diamond,
    SnorkelZombie,
    GoldTrophie,
}

impl TryFrom<u32> for SlotType {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            plant_val if matches!(plant_val, 0..=47 | 52) => {
                SlotType::Plant(plant_val.try_into().unwrap())
            }

            54 => SlotType::ShufflePlants,
            55 => SlotType::Crater,
            56 => SlotType::Sun,
            57 => SlotType::Diamond,
            58 => SlotType::SnorkelZombie,
            59 => SlotType::GoldTrophie,

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

            val => {
                bail!(
                    format!("No discriminant in enum `CardType` matches the value `{val}`")
                        .to_string(),
                );
            }
        })
    }
}

impl TryFrom<SlotType> for u32 {
    type Error = anyhow::Error;

    fn try_from(value: SlotType) -> Result<Self, Self::Error> {
        Ok(match value {
            SlotType::Plant(plant_type) => plant_type.into(),
            SlotType::Zombie(ZombieType::Zombie) => 60,
            SlotType::Zombie(ZombieType::ConeheadZombie) => 61,
            SlotType::Zombie(ZombieType::PoleVaultingZombie) => 62,
            SlotType::Zombie(ZombieType::BucketheadZombie) => 63,
            SlotType::Zombie(ZombieType::LadderZombie) => 64,
            SlotType::Zombie(ZombieType::DiggerZombie) => 65,
            SlotType::Zombie(ZombieType::BungeeZombie) => 66,
            SlotType::Zombie(ZombieType::FootballZombie) => 67,
            SlotType::Zombie(ZombieType::BalloonZombie) => 68,
            SlotType::Zombie(ZombieType::ScreenDoorZombie) => 69,
            SlotType::Zombie(ZombieType::Zomboni) => 70,
            SlotType::Zombie(ZombieType::PogoZombie) => 71,
            SlotType::Zombie(ZombieType::DancingZombie) => 72,
            SlotType::Zombie(ZombieType::GigaGargantuar) => 73,
            SlotType::Zombie(ZombieType::Imp) => 74,
            SlotType::ShufflePlants => 54,
            SlotType::Crater => 55,
            SlotType::Sun => 56,
            SlotType::Diamond => 57,
            SlotType::SnorkelZombie => 58,
            SlotType::GoldTrophie => 59,
            SlotType::Zombie(unknown_zombie) => {
                bail!("Invalid zombie type for card: {unknown_zombie:?}");
            }
        })
    }
}
