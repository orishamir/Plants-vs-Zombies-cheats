use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
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
    PeashooterZombie = 26,
    WallnutZombie = 27,
    JalapenoZombie = 28,
    GatlingPeaZombie = 29,
    SquashZombie = 30,
    TallnutZombie = 31,
    GigaGargantuar = 32,
}

#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum HeadwearType {
    None = 0,
    Cone = 1,
    Bucket = 2,
    Football = 3,
    HardHat = 4,
}

#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum ShieldType {
    None = 0,
    ScreenDoor = 1,
    Newspaper = 2,
    Ladder = 3,
}
