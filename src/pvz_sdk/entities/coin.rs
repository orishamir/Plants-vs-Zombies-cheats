use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::data_array::BoardObject;
use super::{GameObject, Id, PlantType, Position, Ticks};
use crate::il2cpp::{Enum, EnumWithNone, class_layout};

/// Something on the lawn to click and collect, like sun, money or a seed packet. The game
/// calls them all coins.
#[derive(Debug, Clone, PartialEq)]
pub struct Coin {
    pub kind: CoinKind,
    pub position: Position,
    /// How it moves, like falling from the sky or popping out of a plant.
    pub motion: CoinMotion,
    /// How long ago it appeared.
    pub age: Ticks,
    pub is_being_collected: bool,
}

/// What a [`Coin`] is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoinKind {
    Silver,
    Gold,
    Diamond,
    /// Sun worth 25.
    Sun,
    /// Sun worth 15, from a small Sun-shroom.
    SmallSun,
    /// Sun worth 50.
    LargeSun,
    /// The seed packet that ends a level by unlocking a new plant.
    FinalSeedPacket,
    Trophy,
    Shovel,
    Almanac,
    CarKeys,
    Vase,
    WateringCan,
    Taco,
    Note,
    /// A seed packet to plant, like the ones that fall in It's Raining Seeds.
    SeedPacket(PlantType),
    PresentPlant,
    AwardMoneyBag,
    AwardPresent,
    AwardBagDiamond,
    AwardSilverSunflower,
    AwardGoldSunflower,
    Chocolate,
    AwardChocolate,
    PresentMinigames,
    PresentPuzzleMode,
    PresentSurvivalMode,
    DoubleSun,
    Brain,
    VersusTrophyPlant,
    VersusTrophyZombie,
    RipTrophy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CoinMotion {
    FromSky = 0,
    FromSkySlow = 1,
    FromPlant = 2,
    /// Dropped by a zombie.
    Coin = 3,
    /// What an unused lawn mower turns into at the end of a level.
    LawnmowerCoin = 4,
    FromPresent = 5,
    FromBoss = 6,
}

impl Enum for CoinMotion {}

/// The game's `CoinType`: a [`CoinKind`] without the plant in a seed packet, which the game
/// stores separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CoinType {
    Silver = 1,
    Gold = 2,
    Diamond = 3,
    Sun = 4,
    SmallSun = 5,
    LargeSun = 6,
    FinalSeedPacket = 7,
    Trophy = 8,
    Shovel = 9,
    Almanac = 10,
    CarKeys = 11,
    Vase = 12,
    WateringCan = 13,
    Taco = 14,
    Note = 15,
    UsableSeedPacket = 16,
    PresentPlant = 17,
    AwardMoneyBag = 18,
    AwardPresent = 19,
    AwardBagDiamond = 20,
    AwardSilverSunflower = 21,
    AwardGoldSunflower = 22,
    Chocolate = 23,
    AwardChocolate = 24,
    PresentMinigames = 25,
    PresentPuzzleMode = 26,
    PresentSurvivalMode = 27,
    DoubleSun = 28,
    Brain = 29,
    VersusTrophyPlant = 30,
    VersusTrophyZombie = 31,
    RipTrophy = 32, // RIPTrophy
}

impl Enum for CoinType {}

/// `CoinType.None`.
impl EnumWithNone for CoinType {
    const NONE: i32 = 0;
}

class_layout! {
    /// `Reloaded.Gameplay.Coin`.
    pub struct CoinFields = "Reloaded.Gameplay.Coin" {
        id: Id<Coin> = "<DataID>k__BackingField",
        x: f32 = "mPosX",
        y: f32 = "mPosY",
        dead: bool = "mDead",
        age: i32 = "mCoinAge",
        is_being_collected: bool = "mIsBeingCollected",
        coin_type: Option<CoinType> = "mType",
        motion: CoinMotion = "mCoinMotion",
        /// The plant in a `UsableSeedPacket`. Other coins have `SeedType.None` or anything.
        usable_seed_type: Option<PlantType> = "mUsableSeedType",
    }
}

impl GameObject for Coin {
    type Fields = CoinFields;

    fn from_fields(fields: &CoinFields) -> Option<Self> {
        let kind = match fields.coin_type? {
            CoinType::Silver => CoinKind::Silver,
            CoinType::Gold => CoinKind::Gold,
            CoinType::Diamond => CoinKind::Diamond,
            CoinType::Sun => CoinKind::Sun,
            CoinType::SmallSun => CoinKind::SmallSun,
            CoinType::LargeSun => CoinKind::LargeSun,
            CoinType::FinalSeedPacket => CoinKind::FinalSeedPacket,
            CoinType::Trophy => CoinKind::Trophy,
            CoinType::Shovel => CoinKind::Shovel,
            CoinType::Almanac => CoinKind::Almanac,
            CoinType::CarKeys => CoinKind::CarKeys,
            CoinType::Vase => CoinKind::Vase,
            CoinType::WateringCan => CoinKind::WateringCan,
            CoinType::Taco => CoinKind::Taco,
            CoinType::Note => CoinKind::Note,
            CoinType::UsableSeedPacket => CoinKind::SeedPacket(fields.usable_seed_type?),
            CoinType::PresentPlant => CoinKind::PresentPlant,
            CoinType::AwardMoneyBag => CoinKind::AwardMoneyBag,
            CoinType::AwardPresent => CoinKind::AwardPresent,
            CoinType::AwardBagDiamond => CoinKind::AwardBagDiamond,
            CoinType::AwardSilverSunflower => CoinKind::AwardSilverSunflower,
            CoinType::AwardGoldSunflower => CoinKind::AwardGoldSunflower,
            CoinType::Chocolate => CoinKind::Chocolate,
            CoinType::AwardChocolate => CoinKind::AwardChocolate,
            CoinType::PresentMinigames => CoinKind::PresentMinigames,
            CoinType::PresentPuzzleMode => CoinKind::PresentPuzzleMode,
            CoinType::PresentSurvivalMode => CoinKind::PresentSurvivalMode,
            CoinType::DoubleSun => CoinKind::DoubleSun,
            CoinType::Brain => CoinKind::Brain,
            CoinType::VersusTrophyPlant => CoinKind::VersusTrophyPlant,
            CoinType::VersusTrophyZombie => CoinKind::VersusTrophyZombie,
            CoinType::RipTrophy => CoinKind::RipTrophy,
        };
        Some(Self {
            kind,
            position: Position {
                x: fields.x,
                y: fields.y,
            },
            motion: fields.motion,
            age: Ticks(fields.age),
            is_being_collected: fields.is_being_collected,
        })
    }

    fn update_fields(&self, fields: &mut CoinFields) {
        fields.coin_type = Some(match self.kind {
            CoinKind::Silver => CoinType::Silver,
            CoinKind::Gold => CoinType::Gold,
            CoinKind::Diamond => CoinType::Diamond,
            CoinKind::Sun => CoinType::Sun,
            CoinKind::SmallSun => CoinType::SmallSun,
            CoinKind::LargeSun => CoinType::LargeSun,
            CoinKind::FinalSeedPacket => CoinType::FinalSeedPacket,
            CoinKind::Trophy => CoinType::Trophy,
            CoinKind::Shovel => CoinType::Shovel,
            CoinKind::Almanac => CoinType::Almanac,
            CoinKind::CarKeys => CoinType::CarKeys,
            CoinKind::Vase => CoinType::Vase,
            CoinKind::WateringCan => CoinType::WateringCan,
            CoinKind::Taco => CoinType::Taco,
            CoinKind::Note => CoinType::Note,
            CoinKind::SeedPacket(plant) => {
                fields.usable_seed_type = Some(plant);
                CoinType::UsableSeedPacket
            }
            CoinKind::PresentPlant => CoinType::PresentPlant,
            CoinKind::AwardMoneyBag => CoinType::AwardMoneyBag,
            CoinKind::AwardPresent => CoinType::AwardPresent,
            CoinKind::AwardBagDiamond => CoinType::AwardBagDiamond,
            CoinKind::AwardSilverSunflower => CoinType::AwardSilverSunflower,
            CoinKind::AwardGoldSunflower => CoinType::AwardGoldSunflower,
            CoinKind::Chocolate => CoinType::Chocolate,
            CoinKind::AwardChocolate => CoinType::AwardChocolate,
            CoinKind::PresentMinigames => CoinType::PresentMinigames,
            CoinKind::PresentPuzzleMode => CoinType::PresentPuzzleMode,
            CoinKind::PresentSurvivalMode => CoinType::PresentSurvivalMode,
            CoinKind::DoubleSun => CoinType::DoubleSun,
            CoinKind::Brain => CoinType::Brain,
            CoinKind::VersusTrophyPlant => CoinType::VersusTrophyPlant,
            CoinKind::VersusTrophyZombie => CoinType::VersusTrophyZombie,
            CoinKind::RipTrophy => CoinType::RipTrophy,
        });
        fields.x = self.position.x;
        fields.y = self.position.y;
        fields.motion = self.motion;
        fields.age = self.age.0;
        fields.is_being_collected = self.is_being_collected;
    }
}

impl BoardObject for Coin {
    fn id(fields: &CoinFields) -> Id<Coin> {
        fields.id
    }

    fn is_dead(fields: &CoinFields) -> bool {
        fields.dead
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::entity::assert_round_trips;
    use crate::entities::id::test_id;

    #[test]
    fn round_trips() {
        let sun = CoinFields {
            id: test_id(1),
            x: 100.0,
            y: 50.0,
            dead: false,
            age: 10,
            is_being_collected: false,
            coin_type: Some(CoinType::Sun),
            motion: CoinMotion::FromSky,
            usable_seed_type: None,
        };
        let seed_packet = CoinFields {
            id: test_id(2),
            x: 300.0,
            y: 250.0,
            dead: true,
            age: 400,
            is_being_collected: true,
            coin_type: Some(CoinType::UsableSeedPacket),
            motion: CoinMotion::Coin,
            usable_seed_type: Some(PlantType::CherryBomb),
        };
        assert_round_trips::<Coin>(sun.clone(), seed_packet.clone());

        let unpicked_seed_packet = CoinFields {
            usable_seed_type: None,
            ..seed_packet
        };
        assert_eq!(Coin::from_fields(&unpicked_seed_packet), None);
        let new_coin = CoinFields {
            coin_type: None,
            ..sun
        };
        assert_eq!(Coin::from_fields(&new_coin), None);
    }
}
