use crate::ReaderAt;
use crate::models::Lawnmower;
use crate::offsets::LawnmowerOffset;
use crate::traits::ReadableEntity;

impl ReadableEntity for Lawnmower {
    const SIZE: usize = 72;

    fn read(reader: ReaderAt) -> Self {
        assert_eq!(reader.len(), Self::SIZE);

        Self {
            display_pos_x: reader.read_f32(LawnmowerOffset::DisplayPosX).unwrap(),
            display_pos_y: reader.read_f32(LawnmowerOffset::DisplayPosY).unwrap(),
            row: reader.read_u32(LawnmowerOffset::Row).unwrap(),
            mode: reader
                .read_u32(LawnmowerOffset::LawnmowerMode)
                .unwrap()
                .try_into()
                .unwrap(),
            is_deleted: reader.read_bool(LawnmowerOffset::IsDeleted).unwrap(),
            lawnmower_type: reader
                .read_u32(LawnmowerOffset::LawnmowerType)
                .unwrap()
                .try_into()
                .unwrap(),
        }
    }
}
