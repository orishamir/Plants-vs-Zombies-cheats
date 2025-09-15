use crate::Popcapgame;
use crate::entities::{Griditem, GriditemContent, GriditemContentType, VaseContent, VaseKind};
use crate::{offsets::GriditemOffset, writers::WriteableEntity};

impl WriteableEntity for Griditem {
    fn write_entity(&self, addr: usize, game: &Popcapgame) {
        game.write_at(addr, GriditemOffset::IsDeleted, self.is_deleted);

        let griditem_type: GriditemContentType = match self.content {
            GriditemContent::Vase {
                column,
                row,
                is_highlighted,
                opacity,
                vase_kind,
                vase_content,
            } => {
                self.write_vase(
                    addr,
                    game,
                    column,
                    row,
                    is_highlighted,
                    opacity,
                    vase_kind,
                    vase_content,
                );
                GriditemContentType::Vase
            }
            GriditemContent::GraveBuster => GriditemContentType::GraveBuster,
            GriditemContent::DoomShroomCrater => GriditemContentType::DoomShroomCrater,
            GriditemContent::ZenGardenItem => GriditemContentType::ZenGardenItem,
            GriditemContent::Snail {
                pos_x,
                pos_y,
                destination_x,
                destination_y,
            } => {
                game.write_at(addr, GriditemOffset::PosX, pos_x);
                game.write_at(addr, GriditemOffset::PosY, pos_y);
                game.write_at(addr, GriditemOffset::DestinationX, destination_x);
                game.write_at(addr, GriditemOffset::DestinationY, destination_y);
                GriditemContentType::Snail
            }
            GriditemContent::Rake => GriditemContentType::Rake,
            GriditemContent::Brain => GriditemContentType::Brain,
            GriditemContent::Portal => GriditemContentType::Portal,
            GriditemContent::EatableBrain { pos_x, pos_y } => {
                game.write_at(addr, GriditemOffset::PosX, pos_x);
                game.write_at(addr, GriditemOffset::PosY, pos_y);
                GriditemContentType::EatableBrain
            }
        };

        game.write_at::<u32>(addr, GriditemOffset::GriditemType, griditem_type.into());
    }
}

impl Griditem {
    #[allow(clippy::too_many_arguments)]
    fn write_vase(
        &self,
        addr: usize,
        game: &Popcapgame,
        column: u32,
        row: u32,
        is_highlighted: bool,
        opacity: u32,
        vase_kind: VaseKind,
        vase_content: VaseContent,
    ) {
        game.write_at(addr, GriditemOffset::Column, column);
        game.write_at(addr, GriditemOffset::Row, row);
        game.write_at(addr, GriditemOffset::IsHighlighted, is_highlighted);
        game.write_at(addr, GriditemOffset::Opacity, opacity);
        game.write_at::<u32>(
            addr,
            GriditemOffset::VaseKind,
            match vase_kind {
                VaseKind::Mistery => 3,
                VaseKind::Plant => 4,
                VaseKind::Zombie => 5,
            },
        );
        let vase_content_type: u32 = match vase_content {
            VaseContent::Plant { plant_type } => {
                game.write_at::<u32>(addr, GriditemOffset::PlantType, plant_type.into());
                1
            }
            VaseContent::Zombie { zombie_type } => {
                game.write_at::<u32>(addr, GriditemOffset::ZombieType, zombie_type.into());
                2
            }
            VaseContent::Sun { sun_count } => {
                game.write_at(addr, GriditemOffset::SunCount, sun_count);
                3
            }
        };
        game.write_at(addr, GriditemOffset::VaseContentType, vase_content_type);
    }
}
