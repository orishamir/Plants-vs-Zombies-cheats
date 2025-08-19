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

impl From<ZombieType> for u32 {
    fn from(val: ZombieType) -> Self {
        match val {
            ZombieType::Zombie => 0,
            ZombieType::FlagZombie => 1,
            ZombieType::ConeheadZombie => 2,
            ZombieType::PoleVaultingZombie => 3,
            ZombieType::BucketheadZombie => 4,
            ZombieType::NewspaperZombie => 5,
            ZombieType::ScreenDoorZombie => 6,
            ZombieType::FootballZombie => 7,
            ZombieType::DancingZombie => 8,
            ZombieType::BackupDancer => 9,
            ZombieType::DuckyTubeZombie => 10,
            ZombieType::SnorkelZombie => 11,
            ZombieType::Zomboni => 12,
            ZombieType::ZombieBobsledTeam => 13,
            ZombieType::DolphinRiderZombie => 14,
            ZombieType::JackInTheBoxZombie => 15,
            ZombieType::BalloonZombie => 16,
            ZombieType::DiggerZombie => 17,
            ZombieType::PogoZombie => 18,
            ZombieType::ZombieYeti => 19,
            ZombieType::BungeeZombie => 20,
            ZombieType::LadderZombie => 21,
            ZombieType::CatapultZombie => 22,
            ZombieType::Gargantuar => 23,
            ZombieType::Imp => 24,
            ZombieType::DrZomboss => 25,
            ZombieType::GigaGargantuar => 32,
        }
    }
}
