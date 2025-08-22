use std::{fmt::Debug, mem::transmute};

use crate::models::PlantType;

/// Game memory calls this a Coin, but it is any pickable item
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Coin {
    _pad1: [u8; 36],
    pub display_pos_x: f32,
    pub display_pos_y: f32,
    _pad2: [u8; 12],
    pub is_deleted: bool,
    _pad3: [u8; 15],
    pub destination_y: u32,
    pub age_since_spawned: u32,
    _pad5: [u8; 4],
    pub age_since_reached_destination: u32,
    pub coin_type: CoinType,
    _pad6: [u8; 12],
    /// If CoinType is DroppedCard, this is the plant.
    /// For example winning a new card or a broken vase's drop.
    pub plant_type: PlantType,
    _pad7: [u8; 108],
}

impl Debug for Coin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Coin")
            .field("display_pos_x", &{ self.display_pos_x })
            .field("display_pos_y", &{ self.display_pos_y })
            .field("is_deleted", &{ self.is_deleted })
            .field("destination_y", &{ self.destination_y })
            .field("age_since_spawned", &{ self.age_since_spawned })
            .field("age_since_reached_destination", &{
                self.age_since_reached_destination
            })
            .field("coin_type", &{ self.coin_type })
            .field("plant_type", &{ self.plant_type })
            .finish()
    }
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CoinType {
    Silver = 1,
    Gold = 2,
    Sun = 4,
    DroppedCard = 16,
    GiantBagOfCash = 18,
}

impl Debug for CoinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw_value = unsafe { transmute::<&Self, &u32>(self) };
        if !matches!(raw_value, 1 | 2 | 4 | 16 | 18) {
            return write!(f, "{raw_value}");
        }

        match self {
            Self::Silver => write!(f, "Silver"),
            Self::Gold => write!(f, "Gold"),
            Self::Sun => write!(f, "Sun"),
            Self::DroppedCard => write!(f, "DroppedCard"),
            Self::GiantBagOfCash => write!(f, "GiantBagOfCash"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_struct_size() {
        assert_eq!(size_of::<Coin>(), 216);
    }
}
