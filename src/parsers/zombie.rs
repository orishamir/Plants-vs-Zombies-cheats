use crate::models::{ArmorType, Zombie, ZombieType};
use crate::traits::ReadableEntity;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

impl ReadableEntity for Zombie {
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
        rdr.set_position(0xac);
        let freeze_timer = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0xb8);
        let is_hypnotized = rdr.read_u8().unwrap() != 0;
        rdr.set_position(0xc4);
        let armor_type: ArmorType = rdr.read_u32::<LittleEndian>().unwrap().into();
        let health = rdr.read_i32::<LittleEndian>().unwrap();
        let original_health = rdr.read_i32::<LittleEndian>().unwrap();
        let armor_hp = rdr.read_u32::<LittleEndian>().unwrap();
        let original_armor_hp = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0xec);
        let is_dead = rdr.read_u8().unwrap() != 0;

        Self {
            display_pos_x,
            display_pos_y,
            row,
            freeze_timer,
            is_hypnotized,
            zombie_type,
            pos_x,
            pos_y,
            armor_type,
            health,
            original_health,
            original_armor_hp,
            armor_hp,
            is_dead,
        }
    }

    fn size_of() -> usize {
        360
    }
}
