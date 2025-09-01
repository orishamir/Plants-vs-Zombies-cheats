use crate::ReaderAt;
use crate::models::{Griditem, GriditemContent, GriditemContentType, VaseContent, VaseKind};
use crate::offsets::GriditemOffset;
use crate::traits::{ReadEntityError, ReadableEntity};

impl ReadableEntity for Griditem {
    const SIZE: usize = 236;

    fn read(reader: ReaderAt) -> Result<Self, ReadEntityError> {
        assert_eq!(reader.len(), Self::SIZE);

        let is_deleted = reader.read_bool(GriditemOffset::IsDeleted)?;

        let content = match reader.read_u32(GriditemOffset::GriditemType)?.try_into()? {
            GriditemContentType::GraveBuster => GriditemContent::GraveBuster,
            GriditemContentType::DoomShroomCrater => GriditemContent::DoomShroomCrater,
            GriditemContentType::Vase => Self::read_vase(&reader)?,
            GriditemContentType::ZenGardenItem => GriditemContent::ZenGardenItem,
            GriditemContentType::Snail => Self::read_snail(&reader)?,
            GriditemContentType::Rake => GriditemContent::Rake,
            GriditemContentType::Brain => GriditemContent::Brain,
            GriditemContentType::Portal => GriditemContent::Portal,
            GriditemContentType::EatableBrain => GriditemContent::EatableBrain {
                pos_x: reader.read_f32(GriditemOffset::PosX)?,
                pos_y: reader.read_f32(GriditemOffset::PosY)?,
            },
        };

        Ok(Self {
            is_deleted,
            content,
        })
    }
}

impl Griditem {
    fn read_snail(reader: &ReaderAt) -> Result<GriditemContent, ReadEntityError> {
        Ok(GriditemContent::Snail {
            pos_x: reader.read_f32(GriditemOffset::PosX)?,
            pos_y: reader.read_f32(GriditemOffset::PosY)?,
            destination_x: reader.read_f32(GriditemOffset::DestinationX)?,
            destination_y: reader.read_f32(GriditemOffset::DestinationY)?,
        })
    }

    fn read_vase(reader: &ReaderAt) -> Result<GriditemContent, ReadEntityError> {
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
