use strum::IntoEnumIterator;

use crate::{entities::Plant, offsets::PlantOffset, traits::WriteableEntity};

impl WriteableEntity for Plant {
    fn write_entity(&self, addr: usize, game: &crate::game::Popcapgame) {
        for off in PlantOffset::iter() {
            match off {
                PlantOffset::DisplayPosX => game.write_at(addr, off, self.display_pos_x),
                PlantOffset::DisplayPosY => game.write_at(addr, off, self.display_pos_y),
                PlantOffset::Row => game.write_at(addr, off, self.row),
                PlantOffset::PlantType => game.write_at::<u32>(addr, off, self.plant_type.into()),
                PlantOffset::Column => game.write_at(addr, off, self.column),
                PlantOffset::PlantState => game.write_at(addr, off, self.plant_state),
                PlantOffset::Health => game.write_at(addr, off, self.health),
                PlantOffset::OriginalHealth => game.write_at(addr, off, self.original_health),
                PlantOffset::PlantTimer => game.write_at(addr, off, self.plant_timer),
                PlantOffset::HitCount => game.write_at(addr, off, self.hit_counter),
                PlantOffset::IsDeleted => game.write_at(addr, off, self.is_deleted),
                PlantOffset::IsAsleep => game.write_at(addr, off, self.is_asleep),
                PlantOffset::IsConsideredShoveling => {
                    game.write_at(addr, off, self.is_considered_shoveling)
                }
            };
        }
    }
}
