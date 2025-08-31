use crate::models::ProjectileType;
use crate::offsets::ProjectileOffset;
use crate::{models::Projectile, traits::ReadableEntity};

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

impl ReadableEntity for Projectile {
    const SIZE: usize = 148;
    fn from_bytes(buf: &[u8]) -> Self {
        assert_eq!(buf.len(), Self::SIZE);
        let mut rdr = Cursor::new(buf);

        rdr.set_position(ProjectileOffset::DisplayPosX as u64);
        let display_pos_x = rdr.read_u32::<LittleEndian>().unwrap();
        let display_pos_y = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(ProjectileOffset::PosX as u64);
        let pos_x = rdr.read_f32::<LittleEndian>().unwrap();
        let pos_y = rdr.read_f32::<LittleEndian>().unwrap();
        rdr.set_position(ProjectileOffset::CollisionY as u64);
        let collision_y = rdr.read_f32::<LittleEndian>().unwrap();
        let is_deleted = rdr.read_u32::<LittleEndian>().unwrap() != 0;
        rdr.set_position(ProjectileOffset::ProjectileType as u64);
        let projectile_type: ProjectileType = rdr.read_u32::<LittleEndian>().unwrap().into();

        Self {
            display_pos_x,
            display_pos_y,
            pos_x,
            pos_y,
            collision_y,
            is_deleted,
            projectile_type,
        }
    }
}
