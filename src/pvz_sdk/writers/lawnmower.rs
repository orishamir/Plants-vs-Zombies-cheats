use crate::{entities::Lawnmower, offsets::LawnmowerOffset, traits::WriteableEntity};

impl WriteableEntity for Lawnmower {
    fn write_entity(&self, addr: usize, game: &crate::game::Popcapgame) {
        game.write_at(addr, LawnmowerOffset::DisplayPosX, self.display_pos_x);
        game.write_at(addr, LawnmowerOffset::DisplayPosY, self.display_pos_y);
        game.write_at(addr, LawnmowerOffset::Row, self.row);
        game.write_at::<u32>(addr, LawnmowerOffset::LawnmowerMode, self.mode.into());
        game.write_at(addr, LawnmowerOffset::IsDeleted, self.is_deleted);
        game.write_at::<u32>(
            addr,
            LawnmowerOffset::LawnmowerType,
            self.lawnmower_type.into(),
        );
    }
}
