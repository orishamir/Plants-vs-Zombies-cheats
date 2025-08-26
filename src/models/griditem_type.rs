#[allow(dead_code)]
#[derive(Debug)]
pub enum GriditemType {
    Grave,
    DoomShroomCrater,
    Vase,
    Snail,
    Rake,
    /// The brain in the reverse-zombie puzzle thingy
    Brain,
    Unknown(u32),
}

impl From<u32> for GriditemType {
    fn from(value: u32) -> Self {
        match value {
            1 => GriditemType::Grave,
            2 => GriditemType::DoomShroomCrater,
            7 => GriditemType::Vase,
            10 => GriditemType::Snail,
            11 => GriditemType::Rake,
            12 => GriditemType::Brain,
            val => GriditemType::Unknown(val),
        }
    }
}

impl Default for GriditemType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
/// See [Kinds of vases](https://plantsvszombies.fandom.com/wiki/Vasebreaker?file=Scary_Pot.png)
pub enum VaseType {
    /// Normal vase
    Mistery,
    /// The green vase
    Plant,
    /// The zombie vase
    Zombie,
    Unknown(u32),
}

impl Default for VaseType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl From<u32> for VaseType {
    fn from(value: u32) -> Self {
        match value {
            3 => VaseType::Mistery,
            4 => VaseType::Plant,
            5 => VaseType::Zombie,
            val => VaseType::Unknown(val),
        }
    }
}
