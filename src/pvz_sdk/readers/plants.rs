use crate::Popcapgame;
use crate::entities::{Plant, Plants};
use crate::offsets::{EntityInformationOffset, EntityOffset, PlantOffset};
use crate::readers::OffsetReader;
use crate::readers::traits::{ReadEntityError, ReadableEntity};

const PLANT_MEMORY_SIZE: usize = 332;

impl ReadableEntity for Plants {
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
                EntityOffset::Plants.into(),
            ],
            true,
        )?;
        let reader = OffsetReader::new(base_addr, game);

        let capacity = reader.read_u32(EntityInformationOffset::Capacity)?;
        let array_addr = reader.read_usize(EntityInformationOffset::ArrayPtr)?;

        let plants: Vec<Plant> = (0..capacity)
            .filter_map(|i| {
                let entity_reader =
                    OffsetReader::new(array_addr + i as usize * PLANT_MEMORY_SIZE, game);
                Self::read_plant(&entity_reader).ok()
            })
            .filter(|plant| !plant.is_deleted)
            .collect();

        Ok(Self {
            capacity,
            next_index: reader.read_u32(EntityInformationOffset::NextIndex)?,
            count: reader.read_u32(EntityInformationOffset::Count)?,
            plants,
        })
    }
}

impl Plants {
    fn read_plant(reader: &OffsetReader) -> Result<Plant, ReadEntityError> {
        Ok(Plant {
            addr: reader.base_addr,
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
            is_asleep: reader.read_bool(PlantOffset::IsAsleep)?,
            is_considered_shoveling: reader.read_bool(PlantOffset::IsConsideredShoveling)?,
        })
    }
}
