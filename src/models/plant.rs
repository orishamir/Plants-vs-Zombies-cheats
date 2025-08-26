use super::{MemoryParseable, PlantType};
use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    fmt::Debug,
    io::{Cursor, Read, Seek},
};

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Plant {
    pub display_pos_x: u32,
    pub display_pos_y: u32,
    pub row: u32,
    pub plant_type: PlantType,
    pub column: u32,
    pub plant_state: u32,
    pub health: u32,
    pub original_health: u32,
    /// Depends on the plant.
    /// - Chomper: The time left to digest the zombie.
    /// - Cob cannon: The time left until cob is ready to shoot.
    pub plant_timer: u32,
    pub is_deleted: bool,
    pub is_considered_shoveling: bool,
}

impl MemoryParseable for Plant {
    fn from_bytes(buf: Vec<u8>) -> Self {
        assert_eq!(buf.len(), Self::size_of());
        let mut rdr = Cursor::new(buf);

        rdr.set_position(0x8);
        let display_pos_x = rdr.read_u32::<LittleEndian>().unwrap();
        let display_pos_y = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0x1c);
        let row = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0x24);
        let plant_type: PlantType = rdr.read_u32::<LittleEndian>().unwrap().into();
        let column = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0x3c);
        let plant_state = rdr.read_u32::<LittleEndian>().unwrap();
        let health = rdr.read_u32::<LittleEndian>().unwrap();
        let original_health = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0x54);
        let plant_timer = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0x141);
        let is_deleted = rdr.read_u8().unwrap() != 0;
        rdr.set_position(0x145);
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
