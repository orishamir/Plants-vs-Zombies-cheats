use strum::IntoEnumIterator;

use crate::{entities::Lawnmower, offsets::LawnmowerOffset, traits::WriteableEntity};

impl WriteableEntity for Lawnmower {
    fn write_entity(&self, addr: usize, game: &crate::game::Popcapgame) {
        for off in LawnmowerOffset::iter() {
            match off {
                LawnmowerOffset::DisplayPosX => game.write_at(addr, off, self.display_pos_x),
                LawnmowerOffset::DisplayPosY => game.write_at(addr, off, self.display_pos_y),
                LawnmowerOffset::Row => game.write_at(addr, off, self.row),
                LawnmowerOffset::LawnmowerMode => game.write_at::<u32>(addr, off, self.mode.into()),
                LawnmowerOffset::IsDeleted => game.write_at(addr, off, self.is_deleted),
                LawnmowerOffset::LawnmowerType => {
                    game.write_at::<u32>(addr, off, self.lawnmower_type.into())
                }
            };
        }
    }
}
