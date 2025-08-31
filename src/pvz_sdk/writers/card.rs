use crate::{models::Card, offsets::CardOffset, traits::WriteableEntity};

impl WriteableEntity for Card {
    fn write_entity(&self, addr: usize, game: &crate::game::Popcapgame) {
        game.write_at(addr, CardOffset::DisplayPosX, self.display_pos_x);
        game.write_at(addr, CardOffset::DisplayPosY, self.display_pos_y);
        game.write_at(addr, CardOffset::SelectableWidth, self.selectable_width);
        game.write_at(addr, CardOffset::SelectableHeight, self.selectable_height);
        game.write_at(addr, CardOffset::Charge, self.charge);
        game.write_at(addr, CardOffset::RechargeGoal, self.recharge_goal);
        game.write_at(addr, CardOffset::Column, self.column);
        game.write_at::<u32>(
            addr,
            CardOffset::CardType,
            self.card_type.try_into().unwrap(),
        );
        game.write_at(addr, CardOffset::Selectable, self.selectable);
        game.write_at(addr, CardOffset::Recharging, self.recharging);
        game.write_at(addr, CardOffset::UsageCount, self.usage_count);
    }
}
