use crate::ReaderAt;
use crate::models::Plant;
use crate::offsets::PlantOffset;
use crate::traits::{ReadEntityError, ReadableEntity};

impl ReadableEntity for Plant {
    const SIZE: usize = 332;

    fn read(reader: ReaderAt) -> Result<Self, ReadEntityError> {
        assert_eq!(reader.len(), Self::SIZE);

        Ok(Self {
            display_pos_x: reader.read_u32(PlantOffset::DisplayPosX)?,
            display_pos_y: reader.read_u32(PlantOffset::DisplayPosY)?,
            row: reader.read_u32(PlantOffset::Row)?,
            plant_type: reader.read_u32(PlantOffset::PlantType)?.try_into()?,
            column: reader.read_u32(PlantOffset::Column)?,
            plant_state: reader.read_u32(PlantOffset::PlantState)?,
            health: reader.read_u32(PlantOffset::Health)?,
            original_health: reader.read_u32(PlantOffset::OriginalHealth)?,
            plant_timer: reader.read_u32(PlantOffset::PlantTimer)?,
            hit_counter: reader.read_u32(PlantOffset::HitCount)?,
            is_deleted: reader.read_bool(PlantOffset::IsDeleted)?,
            is_considered_shoveling: reader.read_bool(PlantOffset::IsConsideredShoveling)?,
        })
    }
}
