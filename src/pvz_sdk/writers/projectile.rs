use strum::IntoEnumIterator;

use crate::{
    Popcapgame, entities::Projectile, offsets::ProjectileOffset, writers::WriteableEntity,
};

impl WriteableEntity for Projectile {
    fn write_entity(&self, addr: usize, game: &Popcapgame) {
        for off in ProjectileOffset::iter() {
            match off {
                ProjectileOffset::DisplayPosX => game.write_at(addr, off, self.display_pos_x),
                ProjectileOffset::DisplayPosY => game.write_at(addr, off, self.display_pos_y),
                ProjectileOffset::PosX => game.write_at(addr, off, self.pos_x),
                ProjectileOffset::PosY => game.write_at(addr, off, self.pos_y),
                ProjectileOffset::CollisionY => game.write_at(addr, off, self.collision_y),
                ProjectileOffset::IsDeleted => game.write_at(addr, off, self.is_deleted),
                ProjectileOffset::ProjectileType => {
                    game.write_at::<u32>(addr, off, self.projectile_type.into())
                }
            };
        }
    }
}
