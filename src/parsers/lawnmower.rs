use crate::models::{Lawnmower, LawnmowerMode};
use crate::offsets::LawnmowerOffset;
use crate::traits::ReadableEntity;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

impl ReadableEntity for Lawnmower {
    const SIZE: usize = 72;

    fn from_bytes(buf: &[u8]) -> Self {
        assert_eq!(buf.len(), Self::SIZE);
        let mut rdr = Cursor::new(buf);

        rdr.set_position(LawnmowerOffset::DisplayPosX as u64);
        let display_pos_x = rdr.read_f32::<LittleEndian>().unwrap();
        let display_pos_y = rdr.read_f32::<LittleEndian>().unwrap();
        rdr.set_position(LawnmowerOffset::Row as u64);
        let row = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(LawnmowerOffset::LawnmowerMode as u64);
        let mode: LawnmowerMode = rdr.read_u32::<LittleEndian>().unwrap().into();
        let is_deleted = rdr.read_u8().unwrap() != 0;
        rdr.set_position(LawnmowerOffset::LawnmowerType as u64);
        let lawnmower_type = rdr.read_u32::<LittleEndian>().unwrap().into();

        Self {
            display_pos_x,
            display_pos_y,
            row,
            mode,
            is_deleted,
            lawnmower_type,
        }
    }
}
