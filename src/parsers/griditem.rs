use crate::models::{Griditem, GriditemContent, GriditemContentType, VaseContent, VaseKind};
use crate::offsets::GriditemOffset;
use crate::parsers::reader_at::ReaderAt;
use crate::traits::ReadableEntity;

impl ReadableEntity for Griditem {
    const SIZE: usize = 236;

    fn read(reader: ReaderAt) -> Self {
        assert_eq!(reader.len(), Self::SIZE);

        let is_deleted = reader.read_bool(GriditemOffset::IsDeleted).unwrap();

        let content = match reader
            .read_u32(GriditemOffset::GriditemType)
            .unwrap()
            .try_into()
            .unwrap()
        {
            GriditemContentType::GraveBuster => GriditemContent::GraveBuster,
            GriditemContentType::DoomShroomCrater => GriditemContent::DoomShroomCrater,
            GriditemContentType::Vase => Self::read_vase(&reader),
            GriditemContentType::ZenGardenItem => GriditemContent::ZenGardenItem,
            GriditemContentType::Snail => Self::read_snail(&reader),
            GriditemContentType::Rake => GriditemContent::Rake,
            GriditemContentType::Brain => GriditemContent::Brain,
            GriditemContentType::Portal => GriditemContent::Portal,
            GriditemContentType::EatableBrain => GriditemContent::EatableBrain {
                pos_x: reader.read_f32(GriditemOffset::PosX).unwrap(),
                pos_y: reader.read_f32(GriditemOffset::PosY).unwrap(),
            },
        };

        Self {
            is_deleted,
            content,
        }
    }
}

impl Griditem {
    fn read_snail(reader: &ReaderAt) -> GriditemContent {
        GriditemContent::Snail {
            pos_x: reader.read_f32(GriditemOffset::PosX).unwrap(),
            pos_y: reader.read_f32(GriditemOffset::PosY).unwrap(),
            destination_x: reader.read_f32(GriditemOffset::DestinationX).unwrap(),
            destination_y: reader.read_f32(GriditemOffset::DestinationY).unwrap(),
        }
    }

    fn read_vase(reader: &ReaderAt) -> GriditemContent {
        let column = reader.read_u32(GriditemOffset::Column).unwrap();
        let row = reader.read_u32(GriditemOffset::Row).unwrap();

        let is_highlighted = reader.read_bool(GriditemOffset::IsHighlighted).unwrap();
        let opacity = reader.read_u32(GriditemOffset::Opacity).unwrap();

        let vase_kind: VaseKind = match reader.read_u32(GriditemOffset::VaseKind).unwrap() {
            3 => VaseKind::Mistery,
            4 => VaseKind::Plant,
            5 => VaseKind::Zombie,
            _ => unreachable!(),
        };

        let vase_content: VaseContent =
            match reader.read_u32(GriditemOffset::VaseContentType).unwrap() {
                1 => VaseContent::Plant {
                    plant_type: reader.read_u32(GriditemOffset::PlantType).unwrap().into(),
                },
                2 => VaseContent::Zombie {
                    zombie_type: reader.read_u32(GriditemOffset::ZombieType).unwrap().into(),
                },
                // A vase containing suns. The amount is determined by the `sun_count` field
                3 => VaseContent::Sun {
                    sun_count: reader.read_u32(GriditemOffset::SunCount).unwrap(),
                },
                // TODO: Error handling in such cases
                _ => unreachable!(),
            };

        GriditemContent::Vase {
            column,
            row,
            is_highlighted,
            opacity,
            vase_kind,
            vase_content,
        }
    }
}
