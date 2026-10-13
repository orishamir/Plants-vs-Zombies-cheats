use super::{GameObject, PlantType, Ticks, ZombieType};
use crate::il2cpp::{Array, Enum, EnumWithNone, Ref, class_layout};

/// A seed packet in the seed bank at the top of the screen.
#[derive(Debug, Clone, PartialEq)]
pub struct SeedPacket {
    /// What it plants, or `None` for an empty slot, like on a conveyor belt.
    pub seed: Option<Seed>,
    pub state: PacketState,
    /// How long it takes to recharge after it's used.
    pub recharge_time: Ticks,
    pub times_used: i32,
}

/// What a seed packet plants. The game's `SeedType`, together with what an Imitater packet
/// copies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Seed {
    Plant(PlantType),
    /// An Imitater that turns into a copy of this plant.
    Imitater(PlantType),
    /// A zombie to send, like in I, Zombie.
    Zombie(ZombieType),
    /// Beghouled's button that shuffles the plants.
    BeghouledShuffle,
    /// Beghouled's button that fills a crater.
    BeghouledCrater,
    SlotMachineSun,
    SlotMachineDiamond,
    /// A Snorkel Zombie to buy in Zombiquarium.
    ZombiquariumSnorkel,
    /// The trophy to buy, which wins Zombiquarium.
    ZombiquariumTrophy,
}

/// Whether a seed packet can be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PacketState {
    Ready,
    Recharging {
        /// How long it has been recharging, out of its
        /// [`recharge_time`](SeedPacket::recharge_time).
        elapsed: Ticks,
    },
    /// Neither ready nor recharging, like before the level starts.
    Disabled,
}

/// The game's `SeedType` values for the packets that aren't plants. Reading and writing a
/// [`Seed`] both use this, so the two always agree.
const OTHER_SEEDS: [(i32, Seed); 27] = [
    (54, Seed::BeghouledShuffle), // BeghouledButtonShuffle
    (55, Seed::BeghouledCrater),  // BeghouledButtonCrater
    (56, Seed::SlotMachineSun),
    (57, Seed::SlotMachineDiamond),
    (58, Seed::ZombiquariumSnorkel),
    (59, Seed::ZombiquariumTrophy),
    (60, Seed::Zombie(ZombieType::Normal)),   // ZombieNormal
    (61, Seed::Zombie(ZombieType::Conehead)), // ZombieTrafficCone
    (62, Seed::Zombie(ZombieType::PoleVaulting)), // ZombiePolevaulter
    (63, Seed::Zombie(ZombieType::Buckethead)), // ZombiePail
    (64, Seed::Zombie(ZombieType::Ladder)),   // ZombieLadder
    (65, Seed::Zombie(ZombieType::Digger)),   // ZombieDigger
    (66, Seed::Zombie(ZombieType::Bungee)),   // ZombieBungee
    (67, Seed::Zombie(ZombieType::Football)), // ZombieFootball
    (68, Seed::Zombie(ZombieType::Balloon)),  // ZombieBalloon
    (69, Seed::Zombie(ZombieType::ScreenDoor)), // ZombieScreenDoor
    (70, Seed::Zombie(ZombieType::Zomboni)),  // Zomboni
    (71, Seed::Zombie(ZombieType::Pogo)),     // ZombiePogo
    (72, Seed::Zombie(ZombieType::Dancing)),  // ZombieDancer
    (73, Seed::Zombie(ZombieType::Gargantuar)), // ZombieGargantuar
    (74, Seed::Zombie(ZombieType::Imp)),      // ZombieImp
    (75, Seed::Zombie(ZombieType::Flag)),     // ZombieFlag
    (76, Seed::Zombie(ZombieType::TrashCan)), // ZombieTrashCan
    (77, Seed::Zombie(ZombieType::Newspaper)), // ZombieNewspaper
    (78, Seed::Zombie(ZombieType::JackInTheBox)), // ZombieJackInTheBox
    (79, Seed::Zombie(ZombieType::Catapult)), // ZombieCatapult
    (80, Seed::Zombie(ZombieType::Gravestone)), // ZombieGravestone
];

/// Reads a `SeedType`. It's never an [`Imitater`](Seed::Imitater): the game stores that as a
/// plain Imitater packet, and the plant it copies in a separate field.
impl TryFrom<i32> for Seed {
    /// The value that isn't a seed.
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, i32> {
        if let Ok(plant) = PlantType::try_from(value) {
            return Ok(Self::Plant(plant));
        }
        OTHER_SEEDS
            .iter()
            .find(|&&(seed_type, _)| seed_type == value)
            .map(|&(_, seed)| seed)
            .ok_or(value)
    }
}

/// Writes a `SeedType`. An Imitater packet's is `Imitater`, whatever it copies.
impl From<Seed> for i32 {
    fn from(seed: Seed) -> i32 {
        match seed {
            Seed::Plant(plant) => plant.into(),
            Seed::Imitater(_) => PlantType::Imitater.into(),
            other => OTHER_SEEDS
                .iter()
                .find(|&&(_, seed)| seed == other)
                .map(|&(seed_type, _)| seed_type)
                .expect("every seed that isn't a plant is in OTHER_SEEDS"),
        }
    }
}

impl Enum for Seed {}

/// `SeedType.None`.
impl EnumWithNone for Seed {
    const NONE: i32 = -1;
}

class_layout! {
    /// `Reloaded.Gameplay.SeedBank`, the seed packets a player has during a level.
    pub struct SeedBankFields = "Reloaded.Gameplay.SeedBank" {
        packet_count: i32 = "mNumPackets",
        /// Has room for more packets than [`packet_count`](Self::packet_count).
        packets: Ref<Array<Option<Ref<SeedPacketFields>>>> = "mSeedPackets",
    }
}

class_layout! {
    /// `Reloaded.Gameplay.SeedPacket`.
    pub struct SeedPacketFields = "Reloaded.Gameplay.SeedPacket" {
        refresh_counter: i32 = "mRefreshCounter",
        refresh_time: i32 = "mRefreshTime",
        packet_type: Option<Seed> = "mPacketType",
        imitater_type: Option<PlantType> = "mImitaterType",
        active: bool = "mActive",
        refreshing: bool = "mRefreshing",
        times_used: i32 = "mTimesUsed",
    }
}

impl GameObject for SeedPacket {
    type Fields = SeedPacketFields;

    fn from_fields(fields: &SeedPacketFields) -> Option<Self> {
        let seed = match (fields.packet_type, fields.imitater_type) {
            (Some(Seed::Plant(PlantType::Imitater)), Some(copied)) => Some(Seed::Imitater(copied)),
            (seed, _) => seed,
        };
        // `SeedPacket.Update` counts `mRefreshCounter` up while the packet is `mRefreshing`
        // and not `mActive`, and makes it `mActive` when the counter passes `mRefreshTime`.
        let state = match (fields.active, fields.refreshing) {
            (true, _) => PacketState::Ready,
            (false, true) => PacketState::Recharging {
                elapsed: Ticks(fields.refresh_counter),
            },
            (false, false) => PacketState::Disabled,
        };
        Some(Self {
            seed,
            state,
            recharge_time: Ticks(fields.refresh_time),
            times_used: fields.times_used,
        })
    }

    fn update_fields(&self, fields: &mut SeedPacketFields) {
        (fields.packet_type, fields.imitater_type) = match self.seed {
            Some(Seed::Imitater(copied)) => (Some(Seed::Plant(PlantType::Imitater)), Some(copied)),
            seed => (seed, None),
        };
        (fields.active, fields.refreshing) = match self.state {
            PacketState::Ready => (true, false),
            PacketState::Recharging { elapsed } => {
                fields.refresh_counter = elapsed.0;
                (false, true)
            }
            PacketState::Disabled => (false, false),
        };
        fields.refresh_time = self.recharge_time.0;
        fields.times_used = self.times_used;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::entity::assert_round_trips;
    use crate::il2cpp::FieldValue;

    #[test]
    fn seeds_are_read_from_the_games_values() {
        let read = |value: i32| Option::<Seed>::decode(&value.to_le_bytes()).unwrap();
        assert_eq!(read(-1), None);
        assert_eq!(read(2), Some(Seed::Plant(PlantType::CherryBomb)));
        assert_eq!(read(56), Some(Seed::SlotMachineSun));
        assert_eq!(read(77), Some(Seed::Zombie(ZombieType::Newspaper)));
        assert!(Option::<Seed>::decode(&53_i32.to_le_bytes()).is_err());
    }

    #[test]
    fn every_seed_is_written_as_the_value_its_read_from() {
        for (value, seed) in OTHER_SEEDS {
            assert_eq!(i32::from(seed), value);
            assert_eq!(Seed::try_from(value), Ok(seed));
        }
    }

    #[test]
    fn round_trips() {
        let peashooter = SeedPacketFields {
            refresh_counter: 0,
            refresh_time: 900,
            packet_type: Some(Seed::Plant(PlantType::Peashooter)),
            imitater_type: None,
            active: true,
            refreshing: false,
            times_used: 0,
        };
        let imitater = SeedPacketFields {
            refresh_counter: 100,
            refresh_time: 3600,
            packet_type: Some(Seed::Plant(PlantType::Imitater)),
            imitater_type: Some(PlantType::Squash),
            active: false,
            refreshing: true,
            times_used: 3,
        };
        assert_round_trips::<SeedPacket>(peashooter.clone(), imitater);

        let empty = SeedPacketFields {
            refresh_counter: 7,
            refresh_time: 8,
            packet_type: None,
            imitater_type: None,
            active: false,
            refreshing: false,
            times_used: 9,
        };
        assert_round_trips::<SeedPacket>(peashooter, empty);
    }
}
