use crate::{entities::Slot, offsets::SlotOffset, traits::WriteableEntity};

impl WriteableEntity for Slot {
    fn write_entity(&self, addr: usize, game: &crate::game::Popcapgame) {
        game.write_at(addr, SlotOffset::DisplayPosX, self.display_pos_x);
        game.write_at(addr, SlotOffset::DisplayPosY, self.display_pos_y);
        game.write_at(addr, SlotOffset::SelectableWidth, self.selectable_width);
        game.write_at(addr, SlotOffset::SelectableHeight, self.selectable_height);
        game.write_at(addr, SlotOffset::Charge, self.charge);
        game.write_at(addr, SlotOffset::RechargeGoal, self.recharge_goal);
        game.write_at(addr, SlotOffset::Column, self.column);
        game.write_at::<u32>(
            addr,
            SlotOffset::CardType,
            self.card_type.try_into().unwrap(),
        );
        game.write_at(addr, SlotOffset::Selectable, self.selectable);
        game.write_at(addr, SlotOffset::Recharging, self.recharging);
        game.write_at(addr, SlotOffset::UsageCount, self.usage_count);
    }
}
