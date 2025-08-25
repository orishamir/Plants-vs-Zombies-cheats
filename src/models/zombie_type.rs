#[allow(dead_code)]
#[derive(Debug)]
pub enum ZombieType {
    Zombie,
    FlagZombie,
    ConeheadZombie,
    PoleVaultingZombie,
    BucketheadZombie,
    NewspaperZombie,
    ScreenDoorZombie,
    FootballZombie,
    DancingZombie,
    BackupDancer,
    DuckyTubeZombie,
    SnorkelZombie,
    Zomboni,
    ZombieBobsledTeam,
    DolphinRiderZombie,
    JackInTheBoxZombie,
    BalloonZombie,
    DiggerZombie,
    PogoZombie,
    ZombieYeti,
    BungeeZombie,
    LadderZombie,
    CatapultZombie,
    Gargantuar,
    Imp,
    DrZomboss,
    GigaGargantuar,
    Unknown(u32),
}

impl Default for ZombieType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl From<u32> for ZombieType {
    fn from(value: u32) -> Self {
        match value {
            0 => ZombieType::Zombie,
            1 => ZombieType::FlagZombie,
            2 => ZombieType::ConeheadZombie,
            3 => ZombieType::PoleVaultingZombie,
            4 => ZombieType::BucketheadZombie,
            5 => ZombieType::NewspaperZombie,
            6 => ZombieType::ScreenDoorZombie,
            7 => ZombieType::FootballZombie,
            8 => ZombieType::DancingZombie,
            9 => ZombieType::BackupDancer,
            10 => ZombieType::DuckyTubeZombie,
            11 => ZombieType::SnorkelZombie,
            12 => ZombieType::Zomboni,
            13 => ZombieType::ZombieBobsledTeam,
            14 => ZombieType::DolphinRiderZombie,
            15 => ZombieType::JackInTheBoxZombie,
            16 => ZombieType::BalloonZombie,
            17 => ZombieType::DiggerZombie,
            18 => ZombieType::PogoZombie,
            19 => ZombieType::ZombieYeti,
            20 => ZombieType::BungeeZombie,
            21 => ZombieType::LadderZombie,
            22 => ZombieType::CatapultZombie,
            23 => ZombieType::Gargantuar,
            24 => ZombieType::Imp,
            25 => ZombieType::DrZomboss,
            32 => ZombieType::GigaGargantuar,
            val => Self::Unknown(val),
        }
    }
}

#[derive(Debug)]
pub enum ArmorType {
    None,
    Cone,
    Bucket,
    Football,
    Unknown(u32),
}

impl Default for ArmorType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl From<u32> for ArmorType {
    fn from(value: u32) -> Self {
        match value {
            0 => ArmorType::None,
            1 => ArmorType::Cone,
            2 => ArmorType::Bucket,
            3 => ArmorType::Football,
            val => ArmorType::Unknown(val),
        }
    }
}
