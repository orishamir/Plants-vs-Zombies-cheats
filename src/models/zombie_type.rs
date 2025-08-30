use num_enum::{FromPrimitive, IntoPrimitive};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
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

    #[num_enum(catch_all)]
    Unknown(u32),
}

#[derive(Debug, Clone, Copy, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum ArmorType {
    None = 0,
    Cone = 1,
    Bucket = 2,
    Football = 3,
    Digger = 4,
    #[num_enum(catch_all)]
    Unknown(u32),
}
