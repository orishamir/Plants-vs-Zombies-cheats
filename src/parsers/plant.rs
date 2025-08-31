use crate::models::Plant;
use crate::offsets::PlantOffset;
use crate::parsers::reader_at::ReaderAt;
use crate::traits::ReadableEntity;

impl ReadableEntity for Plant {
    const SIZE: usize = 332;

    fn read(reader: ReaderAt) -> Self {
        assert_eq!(reader.len(), Self::SIZE);

        Self {
            display_pos_x: reader.read_u32(PlantOffset::DisplayPosX).unwrap(),
            display_pos_y: reader.read_u32(PlantOffset::DisplayPosY).unwrap(),
            row: reader.read_u32(PlantOffset::Row).unwrap(),
            plant_type: reader.read_u32(PlantOffset::PlantType).unwrap().into(),
            column: reader.read_u32(PlantOffset::Column).unwrap(),
            plant_state: reader.read_u32(PlantOffset::PlantState).unwrap(),
            health: reader.read_u32(PlantOffset::Health).unwrap(),
            original_health: reader.read_u32(PlantOffset::OriginalHealth).unwrap(),
            plant_timer: reader.read_u32(PlantOffset::PlantTimer).unwrap(),
            hit_counter: reader.read_u32(PlantOffset::HitCount).unwrap(),
            is_deleted: reader.read_bool(PlantOffset::IsDeleted).unwrap(),
            is_considered_shoveling: reader
                .read_bool(PlantOffset::IsConsideredShoveling)
                .unwrap(),
        }
    }
}
