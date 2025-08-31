use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

use crate::models::{Coin, CoinContent, CoinType, PlantType};
use crate::offsets::CoinOffset;
use crate::traits::ReadableEntity;

impl ReadableEntity for Coin {
    const SIZE: usize = 216;

    fn from_bytes(buf: &[u8]) -> Self {
        assert_eq!(buf.len(), Self::SIZE);
        let mut rdr = Cursor::new(buf);

        rdr.set_position(CoinOffset::DisplayPosX as u64);
        let display_pos_x = rdr.read_f32::<LittleEndian>().unwrap();
        let display_pos_y = rdr.read_f32::<LittleEndian>().unwrap();
        rdr.set_position(CoinOffset::IsDeleted as u64);
        let is_deleted = rdr.read_u8().unwrap() != 0;
        rdr.set_position(CoinOffset::DestinationY as u64);
        let destination_y = rdr.read_u32::<LittleEndian>().unwrap();
        let age_since_spawned = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(CoinOffset::AgeSinceReachedDestination as u64);
        let age_since_reached_destination = rdr.read_u32::<LittleEndian>().unwrap();
        rdr.set_position(CoinOffset::CoinType as u64);
        let content: CoinContent = match rdr.read_u32::<LittleEndian>().unwrap().try_into().unwrap()
        {
            CoinType::Silver => CoinContent::Silver,
            CoinType::Gold => CoinContent::Gold,
            CoinType::Diamond => CoinContent::Diamond,
            CoinType::Sun => CoinContent::Sun,
            CoinType::MiniSun => CoinContent::MiniSun,
            CoinType::DroppedCard => {
                rdr.set_position(CoinOffset::PlantType as u64);
                let plant_type: PlantType = rdr.read_u32::<LittleEndian>().unwrap().into();
                CoinContent::DroppedCard { plant_type }
            }
            CoinType::GiantBagOfCash => CoinContent::GiantBagOfCash,
            CoinType::GoldSunflowerTrophy => CoinContent::GoldsunflowerTrophy,
            CoinType::Chocolate => CoinContent::Chocolate,
        };

        Self {
            display_pos_x,
            display_pos_y,
            is_deleted,
            destination_y,
            age_since_spawned,
            age_since_reached_destination,
            content,
        }
    }
}
