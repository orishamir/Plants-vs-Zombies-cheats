use super::SlotType;

use std::fmt::Debug;

#[derive(Debug)]
pub struct Slots {
    pub pos_x: i32,
    pub pos_y: i32,
    pub width: u32,
    pub height: u32,
    pub slots_count: u32,
    pub slots: Vec<Slot>,
}

#[derive(Debug)]
pub struct Slot {
    pub addr: usize,

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
    pub card_type: SlotType,
    pub selectable: bool,
    pub recharging: bool,
    pub usage_count: u32,
}
