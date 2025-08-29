use crate::{models::Projectile, offsets::ProjectileOffset, traits::WriteableEntity};

impl WriteableEntity for Projectile {
    fn write_entity(&self, addr: usize, game: &crate::game::Popcapgame) {
        game.write_at(addr, ProjectileOffset::DisplayPosX, self.display_pos_x);
        game.write_at(addr, ProjectileOffset::DisplayPosY, self.display_pos_y);
        game.write_at(addr, ProjectileOffset::PosX, self.pos_x);
        game.write_at(addr, ProjectileOffset::PosY, self.pos_y);
        game.write_at(addr, ProjectileOffset::CollisionY, self.collision_y);
        game.write_at(addr, ProjectileOffset::IsDeleted, self.is_deleted);
        game.write_at::<u32>(
            addr,
            ProjectileOffset::ProjectileType,
            self.projectile_type.into(),
        );
    }
}
