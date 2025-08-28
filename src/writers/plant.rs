use crate::{models::Plant, offsets::PlantOffset, traits::WriteableEntity};

impl WriteableEntity for Plant {
    fn write_entity(&self, addr: usize, game: &crate::game::Popcapgame) {
        game.write_at::<u32>(addr, PlantOffset::DisplayPosX, self.display_pos_x);
        game.write_at::<u32>(addr, PlantOffset::DisplayPosY, self.display_pos_y);
        game.write_at::<u32>(addr, PlantOffset::Row, self.row);
        game.write_at::<u32>(addr, PlantOffset::PlantType, self.plant_type.into());
        game.write_at::<u32>(addr, PlantOffset::Column, self.column);
        game.write_at::<u32>(addr, PlantOffset::PlantState, self.plant_state);
        game.write_at::<u32>(addr, PlantOffset::Health, self.health);
        game.write_at::<u32>(addr, PlantOffset::OriginalHealth, self.original_health);
        game.write_at::<u32>(addr, PlantOffset::PlantTimer, self.plant_timer);
        game.write_at::<bool>(addr, PlantOffset::IsDeleted, self.is_deleted);
        game.write_at::<bool>(
            addr,
            PlantOffset::IsConsideredShoveling,
            self.is_considered_shoveling,
        );
    }
}
