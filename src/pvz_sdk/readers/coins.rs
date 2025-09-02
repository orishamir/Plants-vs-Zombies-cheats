use crate::Popcapgame;
use crate::entities::{Coin, CoinContent, CoinType, Coins};
use crate::offsets::{CoinOffset, EntityInformationOffset, EntityOffset};
use crate::readers::{OffsetReader, ReadEntityError, ReadableEntity};

const COIN_MEMORY_SIZE: usize = 216;

impl ReadableEntity for Coins {
    fn read(game: &Popcapgame) -> Result<Coins, ReadEntityError> {
        let base_addr = game.read_ptr_chain(
            &[
                0x32f39c,
                0x540,
                0x48c,
                0x0,
                0x3dc,
                0x4,
                0x0,
                EntityOffset::Coins.into(),
            ],
            true,
        )?;
        let reader = OffsetReader::new(base_addr, game);

        let capacity = reader.read_u32(EntityInformationOffset::Capacity)?;
        let array_addr = reader.read_usize(EntityInformationOffset::ArrayPtr)?;

        let coins: Vec<Coin> = (0..capacity)
            .filter_map(|i| {
                let entity_reader =
                    OffsetReader::new(array_addr + i as usize * COIN_MEMORY_SIZE, game);
                Self::read_coin(&entity_reader).ok()
            })
            .filter(|coin| !coin.is_deleted)
            .collect();

        Ok(Self {
            capacity,
            next_index: reader.read_u32(EntityInformationOffset::NextIndex)?,
            count: reader.read_u32(EntityInformationOffset::Count)?,
            coins,
        })
    }
}

impl Coins {
    fn read_coin(reader: &OffsetReader) -> Result<Coin, ReadEntityError> {
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

        Ok(Coin {
            addr: reader.base_addr,

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
