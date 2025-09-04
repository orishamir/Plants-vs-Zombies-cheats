use crate::entities;
use crate::{offsets::GriditemOffset, traits::WriteableEntity};

impl WriteableEntity for entities::Griditem {
    fn write_entity(&self, addr: usize, game: &crate::game::Popcapgame) {
        game.write_at(addr, GriditemOffset::IsDeleted, self.is_deleted);

        let griditem_type: entities::GriditemContentType = match self.content {
            entities::GriditemContent::Vase {
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
                entities::GriditemContentType::Vase
            }
            entities::GriditemContent::GraveBuster => entities::GriditemContentType::GraveBuster,
            entities::GriditemContent::DoomShroomCrater => {
                entities::GriditemContentType::DoomShroomCrater
            }
            entities::GriditemContent::ZenGardenItem => {
                entities::GriditemContentType::ZenGardenItem
            }
            entities::GriditemContent::Snail {
                pos_x,
                pos_y,
                destination_x,
                destination_y,
            } => {
                game.write_at(addr, GriditemOffset::PosX, pos_x);
                game.write_at(addr, GriditemOffset::PosY, pos_y);
                game.write_at(addr, GriditemOffset::DestinationX, destination_x);
                game.write_at(addr, GriditemOffset::DestinationY, destination_y);
                entities::GriditemContentType::Snail
            }
            entities::GriditemContent::Rake => entities::GriditemContentType::Rake,
            entities::GriditemContent::Brain => entities::GriditemContentType::Brain,
            entities::GriditemContent::Portal => entities::GriditemContentType::Portal,
            entities::GriditemContent::EatableBrain { pos_x, pos_y } => {
                game.write_at(addr, GriditemOffset::PosX, pos_x);
                game.write_at(addr, GriditemOffset::PosY, pos_y);
                entities::GriditemContentType::EatableBrain
            }
        };

        game.write_at::<u32>(addr, GriditemOffset::GriditemType, griditem_type.into());
    }
}

impl entities::Griditem {
    #[allow(clippy::too_many_arguments)]
    fn write_vase(
        &self,
        addr: usize,
        game: &crate::game::Popcapgame,
        column: u32,
        row: u32,
        is_highlighted: bool,
        opacity: u32,
        vase_kind: entities::VaseKind,
        vase_content: entities::VaseContent,
    ) {
        game.write_at(addr, GriditemOffset::Column, column);
        game.write_at(addr, GriditemOffset::Row, row);
        game.write_at(addr, GriditemOffset::IsHighlighted, is_highlighted);
        game.write_at(addr, GriditemOffset::Opacity, opacity);
        game.write_at::<u32>(
            addr,
            GriditemOffset::VaseKind,
            match vase_kind {
                entities::VaseKind::Mistery => 3,
                entities::VaseKind::Plant => 4,
                entities::VaseKind::Zombie => 5,
            },
        );
        let vase_content_type: u32 = match vase_content {
            crate::entities::VaseContent::Plant { plant_type } => {
                game.write_at::<u32>(addr, GriditemOffset::PlantType, plant_type.into());
                1
            }
            crate::entities::VaseContent::Zombie { zombie_type } => {
                game.write_at::<u32>(addr, GriditemOffset::ZombieType, zombie_type.into());
                2
            }
            crate::entities::VaseContent::Sun { sun_count } => {
                game.write_at(addr, GriditemOffset::SunCount, sun_count);
                3
            }
        };
        game.write_at(addr, GriditemOffset::VaseContentType, vase_content_type);
    }
}
