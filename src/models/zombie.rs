use super::{ArmorType, MemoryParseable, ZombieType};
use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    fmt::Debug,
    io::{Cursor, Read, Seek},
};

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Zombie {
    pub display_pos_x: u32,
    pub display_pos_y: u32,
    pub row: u32,
    pub zombie_type: ZombieType,
    pub pos_x: f32,
    pub pos_y: f32,
    pub armor_type: ArmorType,
    pub health: i32,
    pub armor_hp: u32,
    pub is_dead: bool,
}

impl MemoryParseable for Zombie {
    fn from_bytes(buf: Vec<u8>) -> Self {
        assert_eq!(buf.len(), Self::size_of());
        let mut rdr = Cursor::new(buf);

        rdr.set_position(0x8);
        let display_pos_x = rdr.read_u32::<LittleEndian>().unwrap();
        let display_pos_y = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0x1c);
        let row = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0x24);
        let zombie_type: ZombieType = rdr.read_u32::<LittleEndian>().unwrap().into();
        rdr.set_position(0x2c);
        let pos_x = rdr.read_f32::<LittleEndian>().unwrap();
        let pos_y = rdr.read_f32::<LittleEndian>().unwrap();
        rdr.set_position(0xc4);
        let armor_type: ArmorType = rdr.read_u32::<LittleEndian>().unwrap().into();
        let health = rdr.read_i32::<LittleEndian>().unwrap();
        rdr.set_position(0xd0);
        let armor_hp = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0xec);
        let is_dead = rdr.read_u8().unwrap() != 0;

        Self {
            display_pos_x,
            display_pos_y,
            row,
            zombie_type,
            pos_x,
            pos_y,
            armor_type,
            health,
            armor_hp,
            is_dead,
        }
    }

    fn size_of() -> usize {
        360
    }
}
