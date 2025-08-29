use crate::models::{Plant, PlantType};
use crate::offsets::PlantOffset;
use crate::traits::ReadableEntity;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

impl ReadableEntity for Plant {
    fn from_bytes(buf: Vec<u8>) -> Self {
        assert_eq!(buf.len(), Self::size_of());
        let mut rdr = Cursor::new(buf);

        rdr.set_position(PlantOffset::DisplayPosX as u64);
        let display_pos_x = rdr.read_u32::<LittleEndian>().unwrap();
        let display_pos_y = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(PlantOffset::Row as u64);
        let row = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(PlantOffset::PlantType as u64);
        let plant_type: PlantType = rdr.read_u32::<LittleEndian>().unwrap().into();
        let column = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(PlantOffset::PlantState as u64);
        let plant_state = rdr.read_u32::<LittleEndian>().unwrap();
        let health = rdr.read_u32::<LittleEndian>().unwrap();
        let original_health = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(PlantOffset::PlantTimer as u64);
        let plant_timer = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(PlantOffset::IsDeleted as u64);
        let is_deleted = rdr.read_u8().unwrap() != 0;
        rdr.set_position(PlantOffset::IsConsideredShoveling as u64);
        let is_considered_shoveling = rdr.read_u8().unwrap() != 0;

        Self {
            display_pos_x,
            display_pos_y,
            row,
            plant_type,
            column,
            plant_state,
            health,
            original_health,
            plant_timer,
            is_deleted,
            is_considered_shoveling,
        }
    }

    fn size_of() -> usize {
        332
    }
}
