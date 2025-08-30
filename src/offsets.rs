use num_enum::IntoPrimitive;

#[derive(IntoPrimitive)]
#[repr(usize)]
pub enum PlantOffset {
    DisplayPosX = 0x8,
    DisplayPosY = 0xC,
    Row = 0x1C,
    PlantType = 0x24,
    Column = 0x28,
    PlantState = 0x3C,
    Health = 0x40,
    OriginalHealth = 0x44,
    PlantTimer = 0x54,
    HitCount = 0x58,
    IsDeleted = 0x141,
    IsConsideredShoveling = 0x145,
}

#[derive(IntoPrimitive)]
#[repr(usize)]
pub enum ProjectileOffset {
    DisplayPosX = 0x8,
    DisplayPosY = 0xC,
    PosX = 0x30,
    PosY = 0x34,
    CollisionY = 0x4c,
    IsDeleted = 0x50,
    ProjectileType = 0x5c,
}

#[derive(IntoPrimitive)]
#[repr(usize)]
pub enum GriditemOffset {
    GriditemType = 0x8,
    VaseKind = 0xC,
    Column = 0x10,
    Row = 0x14,
    TimerUntilDead = 0x18,
    IsDeleted = 0x20,
    PosX = 0x24,
    PosY = 0x28,
    DestinationX = 0x2c,
    DestinationY = 0x30,
    ZombieType = 0x3c,
    PlantType = 0x40,
    VaseContentType = 0x44,
    IsHighlighted = 0x48,
    Opacity = 0x4c,
    SunCount = 0x50,
}

#[derive(IntoPrimitive)]
#[repr(usize)]
pub enum CoinOffset {
    DisplayPosX = 0x24,
    DisplayPosY = 0x28,
    IsDeleted = 0x38,
    DestinationY = 0x48,
    AgeSinceSpawned = 0x4c,
    AgeSinceReachedDestination = 0x54,
    CoinType = 0x58,
    PlantType = 0x68,
}

#[derive(IntoPrimitive)]
#[repr(usize)]
pub enum LawnmowerOffset {
    DisplayPosX = 0x8,
    DisplayPosY = 0xc,
    Row = 0x14,
    LawnmowerMode = 0x2c,
    IsDeleted = 0x30,
    LawnmowerType = 0x34,
}

#[derive(IntoPrimitive)]
#[repr(usize)]
pub enum CardOffset {
    DisplayPosX = 0x8,
    DisplayPosY = 0xc,
    SelectableWidth = 0x10,
    SelectableHeight = 0x14,
    Charge = 0x24,
    RechargeGoal = 0x28,
    Column = 0x2c,
    PosXOffset = 0x30,
    CardType = 0x34,
    Selectable = 0x48,
    Recharging = 0x49,
    UsageCount = 0x4c,
}

#[derive(IntoPrimitive)]
#[repr(usize)]
pub enum ZombieOffset {
    DisplayPosX = 0x8,
    DisplayPosY = 0xc,
    Row = 0x1c,
    ZombieType = 0x24,
    PosX = 0x2c,
    PosY = 0x30,
    FreezeTimer = 0xac,
    IsHypnotized = 0xb8,
    ArmorType = 0xc4,
    Health = 0xc8,
    OriginalHealth = 0xcc,
    ArmorHealth = 0xd0,
    OriginalArmorHealth = 0xd4,
    IsDead = 0xec,
}
