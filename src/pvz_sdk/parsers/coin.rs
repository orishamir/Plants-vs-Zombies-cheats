use crate::ReaderAt;
use crate::models::{Coin, CoinContent, CoinType};
use crate::offsets::CoinOffset;
use crate::traits::ReadableEntity;

impl ReadableEntity for Coin {
    const SIZE: usize = 216;

    fn read(reader: ReaderAt) -> Self {
        assert_eq!(reader.len(), Self::SIZE);

        let content: CoinContent = match reader
            .read_u32(CoinOffset::CoinType)
            .unwrap()
            .try_into()
            .unwrap()
        {
            CoinType::Silver => CoinContent::Silver,
            CoinType::Gold => CoinContent::Gold,
            CoinType::Diamond => CoinContent::Diamond,
            CoinType::Sun => CoinContent::Sun,
            CoinType::MiniSun => CoinContent::MiniSun,
            CoinType::DroppedCard => CoinContent::DroppedCard {
                plant_type: reader.read_u32(CoinOffset::PlantType).unwrap().into(),
            },
            CoinType::GiantBagOfCash => CoinContent::GiantBagOfCash,
            CoinType::GoldSunflowerTrophy => CoinContent::GoldsunflowerTrophy,
            CoinType::Chocolate => CoinContent::Chocolate,
        };

        Self {
            content,
            display_pos_x: reader.read_f32(CoinOffset::DisplayPosX).unwrap(),
            display_pos_y: reader.read_f32(CoinOffset::DisplayPosY).unwrap(),
            is_deleted: reader.read_bool(CoinOffset::IsDeleted).unwrap(),
            destination_y: reader.read_u32(CoinOffset::DestinationY).unwrap(),
            age_since_spawned: reader.read_u32(CoinOffset::AgeSinceSpawned).unwrap(),
            age_since_reached_destination: reader
                .read_u32(CoinOffset::AgeSinceReachedDestination)
                .unwrap(),
        }
    }
}
