use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

use crate::models::{Coin, CoinType, PlantType};
use crate::traits::ReadableEntity;

impl ReadableEntity for Coin {
    fn from_bytes(buf: Vec<u8>) -> Self {
        assert_eq!(buf.len(), Self::size_of());
        let mut rdr = Cursor::new(buf);

        rdr.set_position(0x24);
        let display_pos_x = rdr.read_f32::<LittleEndian>().unwrap();
        let display_pos_y = rdr.read_f32::<LittleEndian>().unwrap();
        rdr.set_position(0x38);
        let is_deleted = rdr.read_u8().unwrap() != 0;
        rdr.set_position(0x48);
        let destination_y = rdr.read_u32::<LittleEndian>().unwrap();
        let age_since_spawned = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0x54);
        let age_since_reached_destination = rdr.read_u32::<LittleEndian>().unwrap();
        let coin_type: CoinType = rdr.read_u32::<LittleEndian>().unwrap().into();
        rdr.set_position(0x6b);
        let plant_type: PlantType = rdr.read_u32::<LittleEndian>().unwrap().into();

        Self {
            display_pos_x,
            display_pos_y,
            is_deleted,
            destination_y,
            age_since_spawned,
            age_since_reached_destination,
            coin_type,
            plant_type,
        }
    }

    fn size_of() -> usize {
        216
    }
}
