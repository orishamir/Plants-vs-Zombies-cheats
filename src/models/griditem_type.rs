use num_enum::{FromPrimitive, IntoPrimitive};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum GriditemType {
    Grave = 1,
    DoomShroomCrater = 2,
    Vase = 7,
    /// WateringCan / BugSpray / MusicPlayer / Chocolate
    ZenGardenItem = 9,
    Snail = 10,
    Rake = 11,
    /// The brain in the reverse-zombie puzzle thingy
    Brain = 12,
    #[num_enum(catch_all)]
    Unknown(u32),
}

impl Default for GriditemType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
/// See [Kinds of vases](https://plantsvszombies.fandom.com/wiki/Vasebreaker?file=Scary_Pot.png)
pub enum VaseType {
    /// Normal vase
    Mistery = 3,
    /// The green vase
    Plant = 4,
    /// The zombie vase
    Zombie = 5,
    #[num_enum(catch_all)]
    Unknown(u32),
}

impl Default for VaseType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum VaseContentType {
    /// Normal vase
    Plant = 1,
    /// A vase containing a Zombie
    Zombie = 2,
    /// A vase containing suns. The amount is determined by the `sun_count` field
    Sun = 3,
    #[num_enum(catch_all)]
    Unknown(u32),
}

impl Default for VaseContentType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}
