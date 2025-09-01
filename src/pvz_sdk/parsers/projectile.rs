use crate::ReaderAt;
use crate::offsets::ProjectileOffset;
use crate::traits::ReadEntityError;
use crate::{models::Projectile, traits::ReadableEntity};

impl ReadableEntity for Projectile {
    const SIZE: usize = 148;
    fn read(reader: ReaderAt) -> Result<Self, ReadEntityError> {
        assert_eq!(reader.len(), Self::SIZE);

        Ok(Self {
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
