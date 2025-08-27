#[allow(dead_code)]
#[derive(Debug)]
pub enum CoinType {
    Silver,
    Gold,
    Diamond,
    Sun,
    MiniSun,
    DroppedCard,
    GiantBagOfCash,
    Unknown(u32),
}

impl Default for CoinType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl From<u32> for CoinType {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Silver,
            2 => Self::Gold,
            3 => Self::Diamond,
            4 => Self::Sun,
            5 => Self::MiniSun,
            16 => Self::DroppedCard,
            18 => Self::GiantBagOfCash,
            val => Self::Unknown(val),
        }
    }
}
