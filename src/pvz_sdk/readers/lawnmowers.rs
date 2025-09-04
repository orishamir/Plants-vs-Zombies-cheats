use crate::Popcapgame;
use crate::entities::{Lawnmower, Lawnmowers};
use crate::offsets::{EntityInformationOffset, EntityOffset, LawnmowerOffset};
use crate::readers::OffsetReader;
use crate::readers::traits::{ReadEntityError, ReadableEntity};

const LAWNMOWER_MEMORY_SIZE: usize = 72;

impl ReadableEntity for Lawnmowers {
    fn read(game: &Popcapgame) -> Result<Self, ReadEntityError> {
        let base_addr = game.read_ptr_chain(
            &[
                0x32f39c,
                0x540,
                0x48c,
                0x0,
                0x3dc,
                0x4,
                0x0,
                EntityOffset::Lawnmowers.into(),
            ],
            true,
        )?;
        let reader = OffsetReader::new(base_addr, game);

        let capacity = reader.read_u32(EntityInformationOffset::Capacity)?;
        let array_addr = reader.read_usize(EntityInformationOffset::ArrayPtr)?;

        let lawnmowers: Vec<Lawnmower> = (0..capacity)
            .filter_map(|i| {
                let entity_reader =
                    OffsetReader::new(array_addr + i as usize * LAWNMOWER_MEMORY_SIZE, game);
                Self::read_lawnmower(&entity_reader).ok()
            })
            .filter(|lawnmower| !lawnmower.is_deleted)
            .collect();

        Ok(Self {
            capacity,
            next_index: reader.read_u32(EntityInformationOffset::NextIndex)?,
            count: reader.read_u32(EntityInformationOffset::Count)?,
            lawnmowers,
        })
    }
}

impl Lawnmowers {
    fn read_lawnmower(reader: &OffsetReader) -> Result<Lawnmower, ReadEntityError> {
        Ok(Lawnmower {
            addr: reader.base_addr,
            display_pos_x: reader.read_f32(LawnmowerOffset::DisplayPosX)?,
            display_pos_y: reader.read_f32(LawnmowerOffset::DisplayPosY)?,
            row: reader.read_u32(LawnmowerOffset::Row)?,
            mode: reader
                .read_u32(LawnmowerOffset::LawnmowerMode)?
                .try_into()?,
            is_deleted: reader.read_bool(LawnmowerOffset::IsDeleted)?,
            lawnmower_type: reader
                .read_u32(LawnmowerOffset::LawnmowerType)?
                .try_into()?,
        })
    }
}
