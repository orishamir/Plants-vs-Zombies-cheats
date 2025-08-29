use num_enum::{IntoPrimitive, TryFromPrimitive};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum CoinType {
    Silver = 1,
    Gold = 2,
    Diamond = 3,
    Sun = 4,
    MiniSun = 5,
    DroppedCard = 16,
    GiantBagOfCash = 18,

    #[num_enum(catch_all)]
    Unknown(u32),
}

impl Default for CoinType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}
