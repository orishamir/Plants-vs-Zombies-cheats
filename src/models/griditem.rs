use super::{GriditemType, MemoryParseable, PlantType, VaseType, ZombieType};
use byteorder::{LittleEndian, ReadBytesExt};
use egui::Grid;
use std::{
    fmt::Debug,
    io::{Cursor, Read, Seek},
};

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Griditem {
    pub griditem_type: GriditemType,
    pub vase_type: VaseType,
    pub column: u32,
    pub row: u32,
    pub timer_until_dead: u32,
    pub is_deleted: bool,
    pub zombie_type: ZombieType,
    pub plant_type: PlantType,
    pub is_highlighted: bool,
    pub opacity: u32,
}

impl MemoryParseable for Griditem {
    fn from_bytes(buf: Vec<u8>) -> Self {
        assert_eq!(buf.len(), Self::size_of());
        let mut rdr = Cursor::new(buf);

        rdr.set_position(0x8);
        let griditem_type: GriditemType = rdr.read_u32::<LittleEndian>().unwrap().into();
        let vase_type: VaseType = rdr.read_u32::<LittleEndian>().unwrap().into();
        let column = rdr.read_u32::<LittleEndian>().unwrap();
        let row = rdr.read_u32::<LittleEndian>().unwrap();
        let timer_until_dead = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(0x20);
        let is_deleted = rdr.read_u8().unwrap() != 0;
        rdr.set_position(0x3c);
        let zombie_type: ZombieType = rdr.read_u32::<LittleEndian>().unwrap().into();
        let plant_type: PlantType = rdr.read_u32::<LittleEndian>().unwrap().into();
        rdr.set_position(0x48);
        let is_highlighted = rdr.read_u32::<LittleEndian>().unwrap() != 0;
        let opacity = rdr.read_u32::<LittleEndian>().unwrap();

        Self {
            griditem_type,
            vase_type,
            column,
            row,
            timer_until_dead,
            is_deleted,
            zombie_type,
            plant_type,
            is_highlighted,
            opacity,
        }
    }

    fn size_of() -> usize {
        236
    }
}
