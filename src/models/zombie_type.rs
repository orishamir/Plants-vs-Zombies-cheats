#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
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
}

impl Into<u32> for ZombieType {
    fn into(self) -> u32 {
        match self {
            Self::Zombie => 0,
            Self::FlagZombie => 1,
            Self::ConeheadZombie => 2,
            Self::PoleVaultingZombie => 3,
            Self::BucketheadZombie => 4,
            Self::NewspaperZombie => 5,
            Self::ScreenDoorZombie => 6,
            Self::FootballZombie => 7,
            Self::DancingZombie => 8,
            Self::BackupDancer => 9,
            Self::DuckyTubeZombie => 10,
            Self::SnorkelZombie => 11,
            Self::Zomboni => 12,
            Self::ZombieBobsledTeam => 13,
            Self::DolphinRiderZombie => 14,
            Self::JackInTheBoxZombie => 15,
            Self::BalloonZombie => 16,
            Self::DiggerZombie => 17,
            Self::PogoZombie => 18,
            Self::ZombieYeti => 19,
            Self::BungeeZombie => 20,
            Self::LadderZombie => 21,
            Self::CatapultZombie => 22,
            Self::Gargantuar => 23,
            Self::Imp => 24,
            Self::DrZomboss => 25,
            Self::GigaGargantuar => 32,
        }
    }
}
