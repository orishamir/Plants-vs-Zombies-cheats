use std::fmt::Debug;

use super::{CardType, PlantType};

#[repr(C, packed)]
#[derive(Default, Clone, Copy)]
pub struct Card {
    _pad1: [u8; 8],
    pub display_pos_x: u32,
    pub display_pos_y: u32,
    pub selectable_width: u32,
    pub selectable_height: u32,
    _pad2: [u8; 12],
    pub charge: u32,
    pub recharge_goal: u32,
    pub column: u32,
    _pad3: [u8; 4],
    pub card_type: CardType,
    _pad4: [u8; 16],
    pub selectable: bool,
    pub recharging: bool,
    _pad5: [u8; 2],
    pub usage_count: u32,
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Card")
            .field("display_pos_x", &{ self.display_pos_x })
            .field("display_pos_y", &{ self.display_pos_y })
            .field("selectable_width", &{ self.selectable_width })
            .field("selectable_height", &{ self.selectable_height })
            .field("charge", &{ self.charge })
            .field("recharge_goal", &{ self.recharge_goal })
            .field("column", &{ self.column })
            .field("card_type", &{ self.card_type })
            .field("selectable", &self.selectable)
            .field("recharging", &self.recharging)
            .field("usage_count", &{ self.usage_count })
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_struct_size() {
        assert_eq!(size_of::<Card>(), 80);
    }
}
