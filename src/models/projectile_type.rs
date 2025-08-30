use num_enum::{FromPrimitive, IntoPrimitive};
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum ProjectileType {
    Pea = 0,
    FrozenPea = 1,
    Cabbage = 2,
    Watermelon = 3,
    Puff = 4,
    WinterMelon = 5,
    FlamingPea = 6,
    Star = 7,
    Cacti = 8,
    Basketball = 9,
    Corn = 10,
    GiantCorn = 11,
    Butterstick = 12,
    #[num_enum(catch_all)]
    Unknown(u32),
}
