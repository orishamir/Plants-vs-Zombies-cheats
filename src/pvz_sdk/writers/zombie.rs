use strum::IntoEnumIterator;

use crate::{Popcapgame, entities::Zombie, offsets::ZombieOffset, writers::WriteableEntity};

impl WriteableEntity for Zombie {
    fn write_entity(&self, addr: usize, game: &Popcapgame) {
        for off in ZombieOffset::iter() {
            match off {
                ZombieOffset::DisplayPosX => game.write_at(addr, off, self.display_pos_x),
                ZombieOffset::DisplayPosY => game.write_at(addr, off, self.display_pos_y),
                ZombieOffset::Row => game.write_at(addr, off, self.row),
                ZombieOffset::ZombieType => {
                    game.write_at::<u32>(addr, off, self.zombie_type.into())
                }
                ZombieOffset::PosX => game.write_at(addr, off, self.pos_x),
                ZombieOffset::PosY => game.write_at(addr, off, self.pos_y),
                ZombieOffset::FreezeTimer => game.write_at(addr, off, self.freeze_timer),
                ZombieOffset::IsHypnotized => game.write_at(addr, off, self.is_hypnotized),
                ZombieOffset::ArmorType => game.write_at::<u32>(addr, off, self.armor_type.into()),
                ZombieOffset::Health => game.write_at(addr, off, self.health),
                ZombieOffset::OriginalHealth => game.write_at(addr, off, self.original_health),
                ZombieOffset::ArmorHealth => game.write_at(addr, off, self.armor_hp),
                ZombieOffset::OriginalArmorHealth => {
                    game.write_at(addr, off, self.original_armor_hp)
                }
                ZombieOffset::IsDead => game.write_at(addr, off, self.is_dead),
            };
        }
    }
}
