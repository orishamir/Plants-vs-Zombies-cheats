use crate::Popcapgame;
use crate::entities::{
    Griditem, GriditemContent, GriditemContentType, Griditems, VaseContent, VaseKind,
};
use crate::offsets::{EntityInformationOffset, EntityOffset, GriditemOffset};
use crate::readers::OffsetReader;
use crate::readers::traits::{ReadEntityError, ReadableEntity};

const GRIDITEM_MEMORY_SIZE: usize = 236;

impl ReadableEntity for Griditems {
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
                EntityOffset::Griditems.into(),
            ],
            true,
        )?;
        let reader = OffsetReader::new(base_addr, game);

        let array_addr = reader.read_usize(EntityInformationOffset::ArrayPtr)?;
        let capacity = reader.read_u32(EntityInformationOffset::Capacity)?;

        let griditems: Vec<Griditem> = (0..capacity)
            .filter_map(|i| {
                let entity_reader =
                    OffsetReader::new(array_addr + i as usize * GRIDITEM_MEMORY_SIZE, game);
                Self::read_griditem(&entity_reader).ok()
            })
            .filter(|griditem| !griditem.is_deleted)
            .collect();

        Ok(Self {
            capacity,
            next_index: reader.read_u32(EntityInformationOffset::NextIndex)?,
            count: reader.read_u32(EntityInformationOffset::Count)?,
            griditems,
        })
    }
}

impl Griditems {
    fn read_griditem(reader: &OffsetReader) -> Result<Griditem, ReadEntityError> {
        let is_deleted = reader.read_bool(GriditemOffset::IsDeleted)?;

        let content = match reader.read_u32(GriditemOffset::GriditemType)?.try_into()? {
            GriditemContentType::GraveBuster => GriditemContent::GraveBuster,
            GriditemContentType::DoomShroomCrater => GriditemContent::DoomShroomCrater,
            GriditemContentType::Vase => Self::read_vase(reader)?,
            GriditemContentType::ZenGardenItem => GriditemContent::ZenGardenItem,
            GriditemContentType::Snail => Self::read_snail(reader)?,
            GriditemContentType::Rake => GriditemContent::Rake,
            GriditemContentType::Brain => GriditemContent::Brain,
            GriditemContentType::Portal => GriditemContent::Portal,
            GriditemContentType::EatableBrain => GriditemContent::EatableBrain {
                pos_x: reader.read_f32(GriditemOffset::PosX)?,
                pos_y: reader.read_f32(GriditemOffset::PosY)?,
            },
        };

        Ok(Griditem {
            addr: reader.base_addr,
            is_deleted,
            content,
        })
    }
    fn read_snail(reader: &OffsetReader) -> Result<GriditemContent, ReadEntityError> {
        Ok(GriditemContent::Snail {
            pos_x: reader.read_f32(GriditemOffset::PosX)?,
            pos_y: reader.read_f32(GriditemOffset::PosY)?,
            destination_x: reader.read_f32(GriditemOffset::DestinationX)?,
            destination_y: reader.read_f32(GriditemOffset::DestinationY)?,
        })
    }

    fn read_vase(reader: &OffsetReader) -> Result<GriditemContent, ReadEntityError> {
        let column = reader.read_u32(GriditemOffset::Column)?;
        let row = reader.read_u32(GriditemOffset::Row)?;

        let is_highlighted = reader.read_bool(GriditemOffset::IsHighlighted)?;
        let opacity = reader.read_u32(GriditemOffset::Opacity)?;

        let vase_kind: VaseKind = match reader.read_u32(GriditemOffset::VaseKind)? {
            3 => VaseKind::Mistery,
            4 => VaseKind::Plant,
            5 => VaseKind::Zombie,
            val => {
                return Err(ReadEntityError::UnknownEnumMember(
                    format!("Unknown vase kind: {val}").to_string(),
                ));
            }
        };

        let vase_content: VaseContent = match reader.read_u32(GriditemOffset::VaseContentType)? {
            1 => VaseContent::Plant {
                plant_type: reader.read_u32(GriditemOffset::PlantType)?.try_into()?,
            },
            2 => VaseContent::Zombie {
                zombie_type: reader.read_u32(GriditemOffset::ZombieType)?.try_into()?,
            },
            // A vase containing suns. The amount is determined by the `sun_count` field
            3 => VaseContent::Sun {
                sun_count: reader.read_u32(GriditemOffset::SunCount)?,
            },
            val => {
                return Err(ReadEntityError::UnknownEnumMember(
                    format!("Unknown vase content type: {val}").to_string(),
                ));
            }
        };

        Ok(GriditemContent::Vase {
            column,
            row,
            is_highlighted,
            opacity,
            vase_kind,
            vase_content,
        })
    }
}
