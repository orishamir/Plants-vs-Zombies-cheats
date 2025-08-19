use std::fmt::Debug;

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Coin {
    _pad1: [u8; 36],
    pub display_pos_x: f32,
    pub display_pos_y: f32,
    _pad2: [u8; 12],
    pub is_deleted: u32,
    _pad3: [u8; 12],
    pub destination_y: u32,
    pub age_since_spawned: u32,
    _pad5: [u8; 4],
    pub age_since_reached_destination: u32,
    pub coin_type: CoinType,
    _pad6: [u8; 124],
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
            .finish()
    }
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum CoinType {
    Silver = 1,
    Gold = 2,
    Sun = 4,
    GiantBagOfCash = 18,
}
