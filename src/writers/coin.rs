use crate::{models::Coin, offsets::CoinOffset, traits::WriteableEntity};

impl WriteableEntity for Coin {
    fn write_entity(&self, addr: usize, game: &crate::game::Popcapgame) {
        game.write_at(addr, CoinOffset::DisplayPosX, self.display_pos_x);
        game.write_at(addr, CoinOffset::DisplayPosY, self.display_pos_y);
        game.write_at(addr, CoinOffset::IsDeleted, self.is_deleted);
        game.write_at(addr, CoinOffset::DestinationY, self.destination_y);
        game.write_at(addr, CoinOffset::AgeSinceSpawned, self.age_since_spawned);
        game.write_at(
            addr,
            CoinOffset::AgeSinceReachedDestination,
            self.age_since_reached_destination,
        );
        game.write_at::<u32>(addr, CoinOffset::CoinType, self.coin_type.into());
        game.write_at::<u32>(addr, CoinOffset::PlantType, self.plant_type.into());
    }
}
