use crate::models;
use crate::{offsets::GriditemOffset, traits::WriteableEntity};

impl WriteableEntity for models::Griditem {
    fn write_entity(&self, addr: usize, game: &crate::game::Popcapgame) {
        game.write_at(addr, GriditemOffset::IsDeleted, self.is_deleted);

        let griditem_type: models::GriditemContentType = match self.content {
            models::GriditemContent::Vase {
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
                models::GriditemContentType::Vase
            }
            models::GriditemContent::GraveBuster => models::GriditemContentType::GraveBuster,
            models::GriditemContent::DoomShroomCrater => {
                models::GriditemContentType::DoomShroomCrater
            }
            models::GriditemContent::ZenGardenItem => models::GriditemContentType::ZenGardenItem,
            models::GriditemContent::Snail {
                pos_x,
                pos_y,
                destination_x,
                destination_y,
            } => {
                game.write_at(addr, GriditemOffset::PosX, pos_x);
                game.write_at(addr, GriditemOffset::PosY, pos_y);
                game.write_at(addr, GriditemOffset::DestinationX, destination_x);
                game.write_at(addr, GriditemOffset::DestinationY, destination_y);
                models::GriditemContentType::Snail
            }
            models::GriditemContent::Rake => models::GriditemContentType::Rake,
            models::GriditemContent::Brain => models::GriditemContentType::Brain,
            models::GriditemContent::Portal => models::GriditemContentType::Portal,
            models::GriditemContent::EatableBrain { pos_x, pos_y } => {
                game.write_at(addr, GriditemOffset::PosX, pos_x);
                game.write_at(addr, GriditemOffset::PosY, pos_y);
                models::GriditemContentType::EatableBrain
            }
        };

        game.write_at::<u32>(addr, GriditemOffset::GriditemType, griditem_type.into());
    }
}

impl models::Griditem {
    #[allow(clippy::too_many_arguments)]
    fn write_vase(
        &self,
        addr: usize,
        game: &crate::game::Popcapgame,
        column: u32,
        row: u32,
        is_highlighted: bool,
        opacity: u32,
        vase_kind: models::VaseKind,
        vase_content: models::VaseContent,
    ) {
        game.write_at(addr, GriditemOffset::Column, column);
        game.write_at(addr, GriditemOffset::Row, row);
        game.write_at(addr, GriditemOffset::IsHighlighted, is_highlighted);
        game.write_at(addr, GriditemOffset::Opacity, opacity);
        game.write_at::<u32>(
            addr,
            GriditemOffset::VaseKind,
            match vase_kind {
                models::VaseKind::Mistery => 3,
                models::VaseKind::Plant => 4,
                models::VaseKind::Zombie => 5,
            },
        );
        let vase_content_type: u32 = match vase_content {
            crate::models::VaseContent::Plant { plant_type } => {
                game.write_at::<u32>(addr, GriditemOffset::PlantType, plant_type.into());
                1
            }
            crate::models::VaseContent::Zombie { zombie_type } => {
                game.write_at::<u32>(addr, GriditemOffset::ZombieType, zombie_type.into());
                2
            }
            crate::models::VaseContent::Sun { sun_count } => {
                game.write_at(addr, GriditemOffset::SunCount, sun_count);
                3
            }
        };
        game.write_at(addr, GriditemOffset::VaseContentType, vase_content_type);
    }
}
