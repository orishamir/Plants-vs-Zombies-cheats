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
    IsDeleted = 0x141,
    IsConsideredShoveling = 0x145,
}

impl Into<usize> for PlantOffset {
    fn into(self) -> usize {
        self as usize
    }
}

pub enum ProjectileOffset {
    DisplayPosX = 0x8,
    DisplayPosY = 0xC,
    PosX = 0x30,
    PosY = 0x34,
    CollisionY = 0x4c,
    IsDeleted = 0x50,
    ProjectileType = 0x5c,
}

impl Into<usize> for ProjectileOffset {
    fn into(self) -> usize {
        self as usize
    }
}
pub enum GriditemOffset {
    GriditemType = 0x8,
    VaseType = 0xC,
    Column = 0x10,
    Row = 0x14,
    TimerUntilDead = 0x18,
    IsDeleted = 0x20,
    ZombieType = 0x3c,
    PlantType = 0x40,
    IsHighlighted = 0x48,
    Opacity = 0x4c,
}

impl Into<usize> for GriditemOffset {
    fn into(self) -> usize {
        self as usize
    }
}

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

impl Into<usize> for CoinOffset {
    fn into(self) -> usize {
        self as usize
    }
}

pub enum LawnmowerOffset {
    DisplayPosX = 0x8,
    DisplayPosY = 0xc,
    Row = 0x14,
    LawnmowerMode = 0x2c,
    IsDeleted = 0x30,
    LawnmowerType = 0x34,
}

impl Into<usize> for LawnmowerOffset {
    fn into(self) -> usize {
        self as usize
    }
}

pub enum CardOffset {
    DisplayPosX = 0x8,
    DisplayPosY = 0xc,
    SelectableWidth = 0x10,
    SelectableHeight = 0x14,
    Charge = 0x24,
    RechargeGoal = 0x28,
    Column = 0x2c,
    CardType = 0x34,
    Selectable = 0x48,
    Recharging = 0x49,
    UsageCount = 0x4c,
}

impl Into<usize> for CardOffset {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Into)]
#[into(types(usize, u64))]
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

impl Into<usize> for ZombieOffset {
    fn into(self) -> usize {
        self as usize
    }
}
