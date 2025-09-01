use crate::ReaderAt;
use crate::models::{Coin, CoinContent, CoinType};
use crate::offsets::CoinOffset;
use crate::traits::{ReadEntityError, ReadableEntity};

impl ReadableEntity for Coin {
    const SIZE: usize = 216;

    fn read(reader: ReaderAt) -> Result<Coin, ReadEntityError> {
        assert_eq!(reader.len(), Self::SIZE);

        let content: CoinContent = match reader.read_u32(CoinOffset::CoinType)?.try_into()? {
            CoinType::Silver => CoinContent::Silver,
            CoinType::Gold => CoinContent::Gold,
            CoinType::Diamond => CoinContent::Diamond,
            CoinType::Sun => CoinContent::Sun,
            CoinType::MiniSun => CoinContent::MiniSun,
            CoinType::DroppedCard => CoinContent::DroppedCard {
                plant_type: reader.read_u32(CoinOffset::PlantType)?.try_into()?,
            },
            CoinType::GiantBagOfCash => CoinContent::GiantBagOfCash,
            CoinType::GoldSunflowerTrophy => CoinContent::GoldsunflowerTrophy,
            CoinType::Chocolate => CoinContent::Chocolate,
        };

        Ok(Self {
            content,
            display_pos_x: reader.read_f32(CoinOffset::DisplayPosX)?,
            display_pos_y: reader.read_f32(CoinOffset::DisplayPosY)?,
            is_deleted: reader.read_bool(CoinOffset::IsDeleted)?,
            destination_y: reader.read_u32(CoinOffset::DestinationY)?,
            age_since_spawned: reader.read_u32(CoinOffset::AgeSinceSpawned)?,
            age_since_reached_destination: reader
                .read_u32(CoinOffset::AgeSinceReachedDestination)?,
        })
    }
}
