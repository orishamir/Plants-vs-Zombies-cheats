use std::{fmt::Debug, mem::transmute};

#[allow(dead_code)]
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ZombieType {
    Zombie = 0,
    FlagZombie = 1,
    ConeheadZombie = 2,
    PoleVaultingZombie = 3,
    BucketheadZombie = 4,
    NewspaperZombie = 5,
    ScreenDoorZombie = 6,
    FootballZombie = 7,
    DancingZombie = 8,
    BackupDancer = 9,
    DuckyTubeZombie = 10,
    SnorkelZombie = 11,
    Zomboni = 12,
    ZombieBobsledTeam = 13,
    DolphinRiderZombie = 14,
    JackInTheBoxZombie = 15,
    BalloonZombie = 16,
    DiggerZombie = 17,
    PogoZombie = 18,
    ZombieYeti = 19,
    BungeeZombie = 20,
    LadderZombie = 21,
    CatapultZombie = 22,
    Gargantuar = 23,
    Imp = 24,
    DrZomboss = 25,

    GigaGargantuar = 32,
}

impl Debug for ZombieType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_value = unsafe { transmute::<&Self, &u32>(self) };
        if !matches!(raw_value, 0..=25 | 32) {
            return write!(f, "{raw_value}");
        }

        match self {
            Self::Zombie => write!(f, "Zombie"),
            Self::FlagZombie => write!(f, "FlagZombie"),
            Self::ConeheadZombie => write!(f, "ConeheadZombie"),
            Self::PoleVaultingZombie => write!(f, "PoleVaultingZombie"),
            Self::BucketheadZombie => write!(f, "BucketheadZombie"),
            Self::NewspaperZombie => write!(f, "NewspaperZombie"),
            Self::ScreenDoorZombie => write!(f, "ScreenDoorZombie"),
            Self::FootballZombie => write!(f, "FootballZombie"),
            Self::DancingZombie => write!(f, "DancingZombie"),
            Self::BackupDancer => write!(f, "BackupDancer"),
            Self::DuckyTubeZombie => write!(f, "DuckyTubeZombie"),
            Self::SnorkelZombie => write!(f, "SnorkelZombie"),
            Self::Zomboni => write!(f, "Zomboni"),
            Self::ZombieBobsledTeam => write!(f, "ZombieBobsledTeam"),
            Self::DolphinRiderZombie => write!(f, "DolphinRiderZombie"),
            Self::JackInTheBoxZombie => write!(f, "JackInTheBoxZombie"),
            Self::BalloonZombie => write!(f, "BalloonZombie"),
            Self::DiggerZombie => write!(f, "DiggerZombie"),
            Self::PogoZombie => write!(f, "PogoZombie"),
            Self::ZombieYeti => write!(f, "ZombieYeti"),
            Self::BungeeZombie => write!(f, "BungeeZombie"),
            Self::LadderZombie => write!(f, "LadderZombie"),
            Self::CatapultZombie => write!(f, "CatapultZombie"),
            Self::Gargantuar => write!(f, "Gargantuar"),
            Self::Imp => write!(f, "Imp"),
            Self::DrZomboss => write!(f, "DrZomboss"),
            Self::GigaGargantuar => write!(f, "GigaGargantuar"),
        }
    }
}
