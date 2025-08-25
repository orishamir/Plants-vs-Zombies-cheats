use super::{ArmorType, LawnmowerMode, LawnmowerType, MemoryParseable};
use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    fmt::Debug,
    io::{Cursor, Read, Seek},
};

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Lawnmower {
    pub display_pos_x: f32,
    pub display_pos_y: f32,
    pub row: u32,
    pub lawnmower_mode: LawnmowerMode,
    pub is_deleted: bool,
    pub lawnmower_type: LawnmowerType,
}

impl MemoryParseable for Lawnmower {
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
