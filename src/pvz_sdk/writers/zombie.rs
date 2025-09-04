use crate::{entities::Zombie, offsets::ZombieOffset, traits::WriteableEntity};

impl WriteableEntity for Zombie {
    fn write_entity(&self, addr: usize, game: &crate::game::Popcapgame) {
        game.write_at(addr, ZombieOffset::DisplayPosX, self.display_pos_x);
        game.write_at(addr, ZombieOffset::DisplayPosY, self.display_pos_y);
        game.write_at(addr, ZombieOffset::Row, self.row);
        game.write_at::<u32>(addr, ZombieOffset::ZombieType, self.zombie_type.into());
        game.write_at(addr, ZombieOffset::PosX, self.pos_x);
        game.write_at(addr, ZombieOffset::PosY, self.pos_y);
        game.write_at(addr, ZombieOffset::FreezeTimer, self.freeze_timer);
        game.write_at(addr, ZombieOffset::IsHypnotized, self.is_hypnotized);
        game.write_at::<u32>(addr, ZombieOffset::ArmorType, self.armor_type.into());
        game.write_at(addr, ZombieOffset::Health, self.health);
        game.write_at(addr, ZombieOffset::OriginalHealth, self.original_health);
        game.write_at(addr, ZombieOffset::ArmorHealth, self.armor_hp);
        game.write_at(
            addr,
            ZombieOffset::OriginalArmorHealth,
            self.original_armor_hp,
        );
        game.write_at(addr, ZombieOffset::IsDead, self.is_dead);
    }
}
