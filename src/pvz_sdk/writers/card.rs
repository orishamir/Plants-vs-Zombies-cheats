use strum::IntoEnumIterator;

use crate::{entities::Slot, offsets::SlotOffset, traits::WriteableEntity};

impl WriteableEntity for Slot {
    fn write_entity(&self, addr: usize, game: &crate::game::Popcapgame) {
        for off in SlotOffset::iter() {
            match off {
                SlotOffset::DisplayPosX => game.write_at(addr, off, self.display_pos_x),
                SlotOffset::DisplayPosY => game.write_at(addr, off, self.display_pos_y),
                SlotOffset::SelectableWidth => game.write_at(addr, off, self.selectable_width),
                SlotOffset::SelectableHeight => game.write_at(addr, off, self.selectable_height),
                SlotOffset::Charge => game.write_at(addr, off, self.charge),
                SlotOffset::RechargeGoal => game.write_at(addr, off, self.recharge_goal),
                SlotOffset::Column => game.write_at(addr, off, self.column),
                SlotOffset::PosXOffset => game.write_at(addr, off, self.pos_x_offset),
                SlotOffset::CardType => {
                    game.write_at::<u32>(addr, off, self.card_type.try_into().unwrap())
                }
                SlotOffset::Selectable => game.write_at(addr, off, self.selectable),
                SlotOffset::Recharging => game.write_at(addr, off, self.recharging),
                SlotOffset::UsageCount => game.write_at(addr, off, self.usage_count),
            };
        }
    }
}
