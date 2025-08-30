use crate::models::{Griditem, GriditemContent, GriditemContentType, VaseContent, VaseKind};
use crate::offsets::GriditemOffset;
use crate::traits::ReadableEntity;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

impl ReadableEntity for Griditem {
    fn from_bytes(buf: &[u8]) -> Self {
        assert_eq!(buf.len(), Self::size_of());
        let mut rdr = Cursor::new(buf);

        rdr.set_position(GriditemOffset::IsDeleted as u64);
        let is_deleted = rdr.read_u8().unwrap() != 0;

        rdr.set_position(GriditemOffset::GriditemType as u64);
        let content = match rdr.read_u32::<LittleEndian>().unwrap().try_into().unwrap() {
            GriditemContentType::GraveBuster => GriditemContent::GraveBuster,
            GriditemContentType::DoomShroomCrater => GriditemContent::DoomShroomCrater,
            GriditemContentType::Vase => Self::read_vase(&mut rdr),
            GriditemContentType::ZenGardenItem => GriditemContent::ZenGardenItem,
            GriditemContentType::Snail => Self::read_snail(&mut rdr),
            GriditemContentType::Rake => GriditemContent::Rake,
            GriditemContentType::Brain => GriditemContent::Brain,
            GriditemContentType::Portal => GriditemContent::Portal,
            GriditemContentType::EatableBrain => {
                rdr.set_position(GriditemOffset::PosX as u64);

                GriditemContent::EatableBrain {
                    pos_x: rdr.read_f32::<LittleEndian>().unwrap(),
                    pos_y: rdr.read_f32::<LittleEndian>().unwrap(),
                }
            }
        };

        Self {
            is_deleted,
            content,
        }
    }

    fn size_of() -> usize {
        236
    }
}

impl Griditem {
    fn read_snail(rdr: &mut Cursor<&[u8]>) -> GriditemContent {
        rdr.set_position(GriditemOffset::PosX as u64);

        GriditemContent::Snail {
            pos_x: rdr.read_f32::<LittleEndian>().unwrap(),
            pos_y: rdr.read_f32::<LittleEndian>().unwrap(),
            destination_x: rdr.read_f32::<LittleEndian>().unwrap(),
            destination_y: rdr.read_f32::<LittleEndian>().unwrap(),
        }
    }

    fn read_vase(rdr: &mut Cursor<&[u8]>) -> GriditemContent {
        rdr.set_position(GriditemOffset::Column as u64);
        let column = rdr.read_u32::<LittleEndian>().unwrap();
        let row = rdr.read_u32::<LittleEndian>().unwrap();

        rdr.set_position(GriditemOffset::IsHighlighted as u64);
        let is_highlighted = rdr.read_u32::<LittleEndian>().unwrap() != 0;
        let opacity = rdr.read_u32::<LittleEndian>().unwrap();

        rdr.set_position(GriditemOffset::VaseKind as u64);
        let vase_kind: VaseKind = match rdr.read_u32::<LittleEndian>().unwrap() {
            3 => VaseKind::Mistery,
            4 => VaseKind::Plant,
            5 => VaseKind::Zombie,
            _ => unreachable!(),
        };

        rdr.set_position(GriditemOffset::VaseContentType as u64);
        let vase_content: VaseContent = match rdr.read_u32::<LittleEndian>().unwrap() {
            1 => {
                rdr.set_position(GriditemOffset::PlantType as u64);
                VaseContent::Plant {
                    plant_type: rdr.read_u32::<LittleEndian>().unwrap().into(),
                }
            }
            2 => {
                rdr.set_position(GriditemOffset::ZombieType as u64);
                VaseContent::Zombie {
                    zombie_type: rdr.read_u32::<LittleEndian>().unwrap().into(),
                }
            }
            // A vase containing suns. The amount is determined by the `sun_count` field
            3 => {
                rdr.set_position(GriditemOffset::SunCount as u64);
                VaseContent::Sun {
                    sun_count: rdr.read_u32::<LittleEndian>().unwrap(),
                }
            }
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
