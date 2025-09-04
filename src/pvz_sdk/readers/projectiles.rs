use crate::Popcapgame;
use crate::entities::{Projectile, Projectiles};
use crate::offsets::{EntityInformationOffset, EntityOffset, ProjectileOffset};
use crate::readers::OffsetReader;
use crate::readers::traits::{ReadEntityError, ReadableEntity};

const PROJECTILE_MEMORY_SIZE: usize = 148;

impl ReadableEntity for Projectiles {
    fn read(game: &Popcapgame) -> Result<Self, ReadEntityError> {
        let base_addr = game.read_ptr_chain(
            &[
                0x32f39c,
                0x540,
                0x48c,
                0x0,
                0x3dc,
                0x4,
                0x0,
                EntityOffset::Projectiles.into(),
            ],
            true,
        )?;
        let reader = OffsetReader::new(base_addr, game);

        let capacity = reader.read_u32(EntityInformationOffset::Capacity)?;
        let array_addr = reader.read_usize(EntityInformationOffset::ArrayPtr)?;

        let projectiles: Vec<Projectile> = (0..capacity)
            .filter_map(|i| {
                let entity_reader =
                    OffsetReader::new(array_addr + i as usize * PROJECTILE_MEMORY_SIZE, game);
                Self::read_projectile(&entity_reader).ok()
            })
            .filter(|projectile| !projectile.is_deleted)
            .collect();

        Ok(Self {
            capacity,
            next_index: reader.read_u32(EntityInformationOffset::NextIndex)?,
            count: reader.read_u32(EntityInformationOffset::Count)?,
            projectiles,
        })
    }
}

impl Projectiles {
    fn read_projectile(reader: &OffsetReader) -> Result<Projectile, ReadEntityError> {
        Ok(Projectile {
            addr: reader.base_addr,
            display_pos_x: reader.read_u32(ProjectileOffset::DisplayPosX)?,
            display_pos_y: reader.read_u32(ProjectileOffset::DisplayPosY)?,
            pos_x: reader.read_f32(ProjectileOffset::PosX)?,
            pos_y: reader.read_f32(ProjectileOffset::PosY)?,
            collision_y: reader.read_f32(ProjectileOffset::CollisionY)?,
            is_deleted: reader.read_bool(ProjectileOffset::IsDeleted)?,
            projectile_type: reader
                .read_u32(ProjectileOffset::ProjectileType)?
                .try_into()?,
        })
    }
}
