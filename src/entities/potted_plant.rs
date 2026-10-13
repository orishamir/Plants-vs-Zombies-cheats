use std::time::{Duration, SystemTime};

use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::{Cell, Entity, GameObject, Plant, PlantType};
use crate::Popcapgame;
use crate::il2cpp::{Enum, EnumWithNone, ObjectError, Ref, class_layout};

/// What the Zen Garden keeps about one of its plants between visits, like what it needs to
/// grow. The game's `PottedPlant`; read it with [`potted_plant`](Entity::potted_plant).
#[derive(Debug, Clone, PartialEq)]
pub struct PottedPlant {
    pub plant_type: PlantType,
    /// Which of the Zen Garden's gardens it's in.
    pub garden: GardenType,
    /// Its spot in its garden.
    pub spot: Cell,
    pub age: PottedPlantAge,
    /// What it needs next, if anything.
    pub need: Option<PottedPlantNeed>,
    pub times_fed: i32,
    /// How many times it has to be fed to grow.
    pub feedings_per_grow: i32,
    pub last_watered: Option<SystemTime>,
    pub last_fertilized: Option<SystemTime>,
    /// The last time it got what it needed.
    pub last_need_fulfilled: Option<SystemTime>,
    pub last_chocolate: Option<SystemTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum GardenType {
    Main = 0,
    Mushroom = 1,
    WheelBarrow = 2,
    Aquarium = 3,
}

impl Enum for GardenType {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum PottedPlantAge {
    Sprout = 0,
    Small = 1,
    Medium = 2,
    Full = 3,
}

impl Enum for PottedPlantAge {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum PottedPlantNeed {
    Water = 1,
    Fertilizer = 2,
    BugSpray = 3,
    Phonograph = 4,
}

impl Enum for PottedPlantNeed {}

/// `PottedPlantNeed.None`.
impl EnumWithNone for PottedPlantNeed {
    const NONE: i32 = 0;
}

impl Entity<Plant> {
    /// For a Zen Garden plant, what the Zen Garden keeps about it.
    pub fn potted_plant(
        &self,
        game: &Popcapgame,
    ) -> Result<Option<Entity<PottedPlant>>, ObjectError> {
        let Some(controller) = self.fields.potted_plant_controller else {
            return Ok(None);
        };
        controller
            .read(game)?
            .potted_plant
            .map(|potted_plant| Entity::read(game, potted_plant))
            .transpose()
    }
}

class_layout! {
    /// `Source.Controllers.PottedPlantController`, which shows a Zen Garden plant.
    pub struct PottedPlantControllerFields = "Source.Controllers.PottedPlantController" {
        potted_plant: Option<Ref<PottedPlantFields>> = "<PottedPlant>k__BackingField",
    }
}

class_layout! {
    /// `Reloaded.Gameplay.PottedPlant`.
    pub struct PottedPlantFields = "Reloaded.Gameplay.PottedPlant" {
        seed_type: Option<PlantType> = "mSeedType",
        which_zen_garden: GardenType = "mWhichZenGarden",
        x: i32 = "mX",
        y: i32 = "mY",
        // The times are in seconds since 1970, in UTC (`ZenGarden.Now`, `ZenGarden.epoch`),
        // and 0 for never.
        last_watered_time: i64 = "mLastWateredTime",
        plant_age: PottedPlantAge = "mPlantAge",
        times_fed: i32 = "mTimesFed",
        feedings_per_grow: i32 = "mFeedingsPerGrow",
        plant_need: Option<PottedPlantNeed> = "mPlantNeed",
        last_need_fulfilled_time: i64 = "mLastNeedFulfilledTime",
        last_fertilized_time: i64 = "mLastFertilizedTime",
        last_chocolate_time: i64 = "mLastChocolateTime",
    }
}

/// A time the game stores as seconds since 1970, or `None` for 0, which means never.
fn time(seconds: i64) -> Option<SystemTime> {
    let seconds = u64::try_from(seconds).ok().filter(|&seconds| seconds > 0)?;
    Some(SystemTime::UNIX_EPOCH + Duration::from_secs(seconds))
}

fn seconds(time: Option<SystemTime>) -> i64 {
    time.and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
        .and_then(|since| i64::try_from(since.as_secs()).ok())
        .unwrap_or(0)
}

impl GameObject for PottedPlant {
    type Fields = PottedPlantFields;

    fn from_fields(fields: &PottedPlantFields) -> Option<Self> {
        Some(Self {
            plant_type: fields.seed_type?,
            garden: fields.which_zen_garden,
            spot: Cell {
                row: fields.y,
                column: fields.x,
            },
            age: fields.plant_age,
            need: fields.plant_need,
            times_fed: fields.times_fed,
            feedings_per_grow: fields.feedings_per_grow,
            last_watered: time(fields.last_watered_time),
            last_fertilized: time(fields.last_fertilized_time),
            last_need_fulfilled: time(fields.last_need_fulfilled_time),
            last_chocolate: time(fields.last_chocolate_time),
        })
    }

    fn update_fields(&self, fields: &mut PottedPlantFields) {
        fields.seed_type = Some(self.plant_type);
        fields.which_zen_garden = self.garden;
        fields.y = self.spot.row;
        fields.x = self.spot.column;
        fields.plant_age = self.age;
        fields.plant_need = self.need;
        fields.times_fed = self.times_fed;
        fields.feedings_per_grow = self.feedings_per_grow;
        fields.last_watered_time = seconds(self.last_watered);
        fields.last_fertilized_time = seconds(self.last_fertilized);
        fields.last_need_fulfilled_time = seconds(self.last_need_fulfilled);
        fields.last_chocolate_time = seconds(self.last_chocolate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::entity::assert_round_trips;

    #[test]
    fn round_trips() {
        let new_sprout = PottedPlantFields {
            seed_type: Some(PlantType::Marigold),
            which_zen_garden: GardenType::Main,
            x: 0,
            y: 0,
            last_watered_time: 0,
            plant_age: PottedPlantAge::Sprout,
            times_fed: 0,
            feedings_per_grow: 3,
            plant_need: None,
            last_need_fulfilled_time: 0,
            last_fertilized_time: 0,
            last_chocolate_time: 0,
        };
        let grown = PottedPlantFields {
            seed_type: Some(PlantType::KernelPult),
            which_zen_garden: GardenType::Mushroom,
            x: 3,
            y: 1,
            last_watered_time: 1_760_000_000,
            plant_age: PottedPlantAge::Full,
            times_fed: 5,
            feedings_per_grow: 4,
            plant_need: Some(PottedPlantNeed::BugSpray),
            last_need_fulfilled_time: 1_760_000_100,
            last_fertilized_time: 1_760_000_200,
            last_chocolate_time: 1_760_000_300,
        };
        assert_round_trips::<PottedPlant>(new_sprout, grown);
    }

    #[test]
    fn times_are_seconds_since_1970() {
        assert_eq!(time(0), None);
        let time = time(86_400).unwrap();
        assert_eq!(
            time.duration_since(SystemTime::UNIX_EPOCH).unwrap(),
            Duration::from_secs(86_400)
        );
        assert_eq!(seconds(Some(time)), 86_400);
        assert_eq!(seconds(None), 0);
    }
}
