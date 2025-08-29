use crate::{models::Griditem, offsets::GriditemOffset, traits::WriteableEntity};

impl WriteableEntity for Griditem {
    fn write_entity(&self, addr: usize, game: &crate::game::Popcapgame) {
        game.write_at::<u32>(
            addr,
            GriditemOffset::GriditemType,
            self.griditem_type.into(),
        );
        game.write_at::<u32>(addr, GriditemOffset::VaseType, self.vase_type.into());
        game.write_at(addr, GriditemOffset::Column, self.column);
        game.write_at(addr, GriditemOffset::Row, self.row);
        game.write_at(addr, GriditemOffset::TimerUntilDead, self.timer_until_dead);
        game.write_at(addr, GriditemOffset::IsDeleted, self.is_deleted);
        game.write_at::<u32>(addr, GriditemOffset::ZombieType, self.zombie_type.into());
        game.write_at::<u32>(addr, GriditemOffset::PlantType, self.plant_type.into());
        game.write_at(addr, GriditemOffset::IsHighlighted, self.is_highlighted);
        game.write_at(addr, GriditemOffset::Opacity, self.opacity);
    }
}
