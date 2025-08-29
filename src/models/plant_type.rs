use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum PlantType {
    Peashooter = 0,
    Sunflower = 1,
    CherryBomb = 2,
    WallNut = 3,
    PotatoMine = 4,
    SnowPea = 5,
    Chomper = 6,
    Repeater = 7,
    PuffShroom = 8,
    SunShroom = 9,
    FumeShroom = 10,
    GraveBuster = 11,
    HypnoShroom = 12,
    ScaredyShroom = 13,
    IceShroom = 14,
    DoomShroom = 15,
    LilyPad = 16,
    Squash = 17,
    Threepeater = 18,
    TangleKelp = 19,
    Jalapeno = 20,
    Spikeweed = 21,
    Torchwood = 22,
    TallNut = 23,
    SeaShroom = 24,
    Plantern = 25,
    Cactus = 26,
    Blover = 27,
    SplitPea = 28,
    Starfruit = 29,
    Pumpkin = 30,
    MagnetShroom = 31,
    CabbagePult = 32,
    FlowerPot = 33,
    KernelPult = 34,
    CoffeeBean = 35,
    Garlic = 36,
    UmbrellaLeaf = 37,
    Marigold = 38,
    MelonPult = 39,
    GatlingPea = 40,
    TwinSunflower = 41,
    GloomShroom = 42,
    Cattail = 43,
    WinterMelon = 44,
    GoldMagnet = 45,
    Spikerock = 46,
    CobCannon = 47,
    ReverseRepeater = 52,

    #[num_enum(catch_all)]
    Unknown(u32),
}

impl Default for PlantType {
    fn default() -> Self {
        Self::Unknown(255)
    }
}
