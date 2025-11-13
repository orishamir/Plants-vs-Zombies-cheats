use crate::Popcapgame;
use crate::entities::{CoinContent, CoinType};
use crate::{entities::Coin, offsets::CoinOffset, writers::WriteableEntity};

impl WriteableEntity for Coin {
    fn write_entity(&self, addr: usize, game: &Popcapgame) {
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
        let coin_type = match self.content {
            CoinContent::Silver => CoinType::Silver,
            CoinContent::Gold => CoinType::Gold,
            CoinContent::Diamond => CoinType::Diamond,
            CoinContent::Sun => CoinType::Sun,
            CoinContent::MiniSun => CoinType::MiniSun,
            CoinContent::DroppedCard { plant_type } => {
                game.write_at::<u32>(addr, CoinOffset::PlantType, plant_type.into());
                CoinType::DroppedCard
            }
            CoinContent::GiantBagOfCash => CoinType::GiantBagOfCash,
            CoinContent::GoldsunflowerTrophy => CoinType::GoldSunflowerTrophy,
            CoinContent::Chocolate => CoinType::Chocolate,
            CoinContent::BigSun => CoinType::BigSun,
        };
        game.write_at::<u32>(addr, CoinOffset::CoinType, coin_type.into());
    }
}
