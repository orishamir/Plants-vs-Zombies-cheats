use crate::ReaderAt;
use crate::models::Lawnmower;
use crate::offsets::LawnmowerOffset;
use crate::traits::{ReadEntityError, ReadableEntity};

impl ReadableEntity for Lawnmower {
    const SIZE: usize = 72;

    fn read(reader: ReaderAt) -> Result<Self, ReadEntityError> {
        assert_eq!(reader.len(), Self::SIZE);

        Ok(Self {
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
