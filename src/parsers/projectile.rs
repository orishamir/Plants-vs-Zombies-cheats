use crate::offsets::ProjectileOffset;
use crate::parsers::reader_at::ReaderAt;
use crate::{models::Projectile, traits::ReadableEntity};

impl ReadableEntity for Projectile {
    const SIZE: usize = 148;
    fn read(reader: ReaderAt) -> Self {
        assert_eq!(reader.len(), Self::SIZE);

        Self {
            display_pos_x: reader.read_u32(ProjectileOffset::DisplayPosX).unwrap(),
            display_pos_y: reader.read_u32(ProjectileOffset::DisplayPosY).unwrap(),
            pos_x: reader.read_f32(ProjectileOffset::PosX).unwrap(),
            pos_y: reader.read_f32(ProjectileOffset::PosY).unwrap(),
            collision_y: reader.read_f32(ProjectileOffset::CollisionY).unwrap(),
            is_deleted: reader.read_bool(ProjectileOffset::IsDeleted).unwrap(),
            projectile_type: reader
                .read_u32(ProjectileOffset::ProjectileType)
                .unwrap()
                .into(),
        }
    }
}
