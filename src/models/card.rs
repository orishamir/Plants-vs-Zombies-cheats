use super::CardType;

use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Card {
    pub display_pos_x: u32,
    pub display_pos_y: u32,
    pub selectable_width: u32,
    pub selectable_height: u32,
    pub charge: u32,
    pub recharge_goal: u32,
    pub column: u32,
    /// For a conveyor belt level, this value starts at ~1000 and slowly gets decreased
    /// When rendering, it is added to [display_pos_x].
    pub pos_x_offset: i32,
    pub card_type: CardType,
    pub selectable: bool,
    pub recharging: bool,
    pub usage_count: u32,
}
