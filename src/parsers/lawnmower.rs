use crate::models::{Lawnmower, LawnmowerMode};
use crate::traits::ReadableEntity;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

impl ReadableEntity for Lawnmower {
    fn from_bytes(buf: Vec<u8>) -> Self {
        assert_eq!(buf.len(), Self::size_of());
        let mut rdr = Cursor::new(buf);

        rdr.set_position(0x8);
        let display_pos_x = rdr.read_f32::<LittleEndian>().unwrap();
        let display_pos_y = rdr.read_f32::<LittleEndian>().unwrap();
        rdr.set_position(0x14);
        let row = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0x2c);
        let lawnmower_mode: LawnmowerMode = rdr.read_u32::<LittleEndian>().unwrap().into();
        let is_deleted = rdr.read_u8().unwrap() != 0;
        rdr.set_position(0x34);
        let lawnmower_type = rdr.read_u32::<LittleEndian>().unwrap().into();

        Self {
            display_pos_x,
            display_pos_y,
            row,
            lawnmower_mode,
            is_deleted,
            lawnmower_type,
        }
    }

    fn size_of() -> usize {
        72
    }
}
