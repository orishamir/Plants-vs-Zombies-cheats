use crate::models::{Card, CardType};
use crate::traits::ReadableEntity;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

impl ReadableEntity for Card {
    fn from_bytes(buf: Vec<u8>) -> Self {
        assert_eq!(buf.len(), Self::size_of());
        let mut rdr = Cursor::new(buf);

        rdr.set_position(0x8);
        let display_pos_x = rdr.read_u32::<LittleEndian>().unwrap();
        let display_pos_y = rdr.read_u32::<LittleEndian>().unwrap();
        let selectable_width = rdr.read_u32::<LittleEndian>().unwrap();
        let selectable_height = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0x24);
        let charge = rdr.read_u32::<LittleEndian>().unwrap();
        let recharge_goal = rdr.read_u32::<LittleEndian>().unwrap();
        let column = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0x34);
        let card_type: CardType = rdr.read_u32::<LittleEndian>().unwrap().into();
        rdr.set_position(0x48);
        let selectable = rdr.read_u8().unwrap() != 0;
        let recharging = rdr.read_u8().unwrap() != 0;
        rdr.set_position(0x4c);
        let usage_count = rdr.read_u32::<LittleEndian>().unwrap();

        Self {
            display_pos_x,
            display_pos_y,
            selectable_width,
            selectable_height,
            charge,
            recharge_goal,
            column,
            card_type,
            selectable,
            recharging,
            usage_count,
        }
    }

    fn size_of() -> usize {
        80
    }
}
