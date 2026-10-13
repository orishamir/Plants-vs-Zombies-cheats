use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::data_array::BoardObject;
use super::potted_plant::PottedPlantControllerFields;
use super::{Cell, GameObject, Health, Id, Position, Ticks, Zombie};
use crate::il2cpp::{Enum, EnumWithNone, Ref, class_layout};

/// A plant on the lawn.
#[derive(Debug, Clone, PartialEq)]
pub struct Plant {
    pub plant_type: PlantType,
    pub cell: Cell,
    /// Where it's drawn.
    pub position: Position,
    pub health: Health,
    /// What it's doing, like a Chomper that's digesting.
    pub state: PlantState,
    /// How long until the plant's [`state`](Self::state) changes, for states that end on
    /// their own. For example, how long a Chomper has left to digest.
    pub state_countdown: Ticks,
    /// How long until it next acts, like shooting or making sun. Some plants that do neither,
    /// like the Plantern, have one too.
    pub action_countdown: Ticks,
    /// About how long it waits between those actions.
    pub action_interval: Ticks,
    /// For a plant that acts once, like a Cherry Bomb, how long until it does, or 0 if it
    /// isn't about to.
    pub special_countdown: Ticks,
    /// A mushroom sleeps in the day, unless a Coffee Bean wakes it.
    pub is_asleep: bool,
    pub grabbed_by_bungee: Option<BungeeGrab>,
    /// For an Imitater, the plant it's turning into. The plant an Imitater turned into has
    /// `Some(PlantType::Imitater)` here.
    pub imitater_type: Option<PlantType>,
    /// Whether an Imitater turned into this plant, which makes it gray.
    pub is_imitation: bool,
    /// The zombie a Squash is about to squash, or that a Tangle Kelp grabbed.
    pub target_zombie: Option<Id<Zombie>>,
}

/// The game's `SeedType`, without the seed packets that aren't plants (see
/// [`Seed`](super::Seed)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum PlantType {
    Peashooter = 0,
    Sunflower = 1,
    CherryBomb = 2,
    WallNut = 3,
    PotatoMine = 4,
    SnowPea = 5,
    Chomper = 6,
    Repeater = 7,
    PuffShroom = 8,
    SunShroom = 9,
    FumeShroom = 10,
    GraveBuster = 11,
    HypnoShroom = 12,
    ScaredyShroom = 13,
    IceShroom = 14,
    DoomShroom = 15,
    LilyPad = 16,
    Squash = 17,
    Threepeater = 18,
    TangleKelp = 19,
    Jalapeno = 20,
    Spikeweed = 21,
    Torchwood = 22,
    TallNut = 23,
    SeaShroom = 24,
    Plantern = 25,
    Cactus = 26,
    Blover = 27,
    SplitPea = 28,
    Starfruit = 29,
    Pumpkin = 30, // Pumpkinshell
    MagnetShroom = 31,
    CabbagePult = 32,
    FlowerPot = 33,
    KernelPult = 34,
    CoffeeBean = 35, // InstantCoffee
    Garlic = 36,
    UmbrellaLeaf = 37, // Umbrella
    Marigold = 38,
    MelonPult = 39,
    GatlingPea = 40,
    TwinSunflower = 41,
    GloomShroom = 42,
    Cattail = 43,
    WinterMelon = 44,
    GoldMagnet = 45,
    Spikerock = 46,
    CobCannon = 47,
    Imitater = 48,
    /// The red Wall-nut of Wall-nut Bowling, which explodes.
    ExplodeONut = 49,
    /// The huge Wall-nut of Wall-nut Bowling 2.
    GiantWallNut = 50,
    /// A Zen Garden plant that hasn't grown up yet.
    Sprout = 51,
    /// What some of Vasebreaker's vases hold (`Challenge.ScaryPotterPopulate`).
    ReverseRepeater = 52, // Leftpeater
}

impl Enum for PlantType {}

/// `SeedType.None`.
impl EnumWithNone for PlantType {
    const NONE: i32 = -1;
}

/// What a plant is doing. Most plants are only ever [`Ready`](Self::Ready); the rest of the
/// states are each for one kind of plant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum PlantState {
    NotReady = 0,
    Ready = 1,
    DoingSpecial = 2,
    SquashLook = 3,
    SquashPreLaunch = 4,
    SquashRising = 5,
    SquashFalling = 6,
    SquashDoneFalling = 7,
    GraveBusterLanding = 8,
    GraveBusterEating = 9,
    ChomperBiting = 10,
    ChomperBitingGotOne = 11,
    ChomperBitingMissed = 12,
    ChomperDigesting = 13,
    ChomperSwallowing = 14,
    PotatoRising = 15,
    PotatoArmed = 16,
    PotatoMashed = 17,
    SpikeweedAttacking = 18,
    SpikeweedAttacking2 = 19,
    ScaredyShroomLowering = 20,
    ScaredyShroomScared = 21,
    ScaredyShroomRaising = 22,
    SunShroomSmall = 23,
    SunShroomGrowing = 24,
    SunShroomBig = 25,
    MagnetShroomSucking = 26,
    MagnetShroomCharging = 27,
    BowlingUp = 28,
    BowlingDown = 29,
    CactusLow = 30,
    CactusRising = 31,
    CactusHigh = 32,
    CactusLowering = 33,
    TangleKelpGrabbing = 34,
    CobCannonArming = 35,
    CobCannonLoading = 36,
    CobCannonReady = 37,
    CobCannonFiring = 38,
    KernelPultButter = 39,
    UmbrellaTriggered = 40,
    UmbrellaReflecting = 41,
    ImitaterMorphing = 42,
    ZenGardenWatered = 43,
    ZenGardenNeedy = 44,
    ZenGardenHappy = 45,
    MarigoldEnding = 46,
    FlowerPotInvulnerable = 47,
    LilyPadInvulnerable = 48,
}

impl Enum for PlantState {}

/// A Bungee Zombie taking a plant away. The game's `PlantOnBungeeState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum BungeeGrab {
    BeingGrabbed = 1, // GettingGrabbedByBungee
    Rising = 2,       // RisingWithBungee
}

impl Enum for BungeeGrab {}

/// `PlantOnBungeeState.NotOnBungee`.
impl EnumWithNone for BungeeGrab {
    const NONE: i32 = 0;
}

class_layout! {
    /// `Reloaded.Gameplay.Plant`.
    pub struct PlantFields = "Reloaded.Gameplay.Plant" {
        id: Id<Plant> = "<DataID>k__BackingField",
        x: f32 = "mX",
        y: f32 = "mY",
        row: i32 = "mRow",
        /// `Plant.Reset` sets it to `SeedType.None`.
        plant_type: Option<PlantType> = "mSeedType",
        column: i32 = "mPlantCol",
        state: PlantState = "mState",
        health: i32 = "mPlantHealth",
        max_health: i32 = "mPlantMaxHealth",
        do_special_countdown: i32 = "mDoSpecialCountdown",
        state_countdown: i32 = "mStateCountdown",
        launch_counter: i32 = "mLaunchCounter",
        launch_rate: i32 = "mLaunchRate",
        on_bungee_state: Option<BungeeGrab> = "mOnBungeeState",
        imitater_type: Option<PlantType> = "mImitaterType",
        dead: bool = "mDead",
        is_asleep: bool = "mIsAsleep",
        is_imitation: bool = "mIsImitation",
        /// Only the Squash and the Tangle Kelp set it (`Plant.UpdateSquash`, `UpdateTanglekelp`).
        target_zombie_id: Option<Id<Zombie>> = "mTargetZombieID",
        /// Only Zen Garden plants have one.
        potted_plant_controller: Option<Ref<PottedPlantControllerFields>> = "PottedPlantController",
    }
}

impl GameObject for Plant {
    type Fields = PlantFields;

    fn from_fields(fields: &PlantFields) -> Option<Self> {
        Some(Self {
            plant_type: fields.plant_type?,
            cell: Cell {
                row: fields.row,
                column: fields.column,
            },
            position: Position {
                x: fields.x,
                y: fields.y,
            },
            health: Health {
                current: fields.health,
                max: fields.max_health,
            },
            state: fields.state,
            state_countdown: Ticks(fields.state_countdown),
            action_countdown: Ticks(fields.launch_counter),
            action_interval: Ticks(fields.launch_rate),
            special_countdown: Ticks(fields.do_special_countdown),
            is_asleep: fields.is_asleep,
            grabbed_by_bungee: fields.on_bungee_state,
            imitater_type: fields.imitater_type,
            is_imitation: fields.is_imitation,
            target_zombie: fields.target_zombie_id,
        })
    }

    fn update_fields(&self, fields: &mut PlantFields) {
        fields.plant_type = Some(self.plant_type);
        fields.row = self.cell.row;
        fields.column = self.cell.column;
        fields.x = self.position.x;
        fields.y = self.position.y;
        fields.health = self.health.current;
        fields.max_health = self.health.max;
        fields.state = self.state;
        fields.state_countdown = self.state_countdown.0;
        fields.launch_counter = self.action_countdown.0;
        fields.launch_rate = self.action_interval.0;
        fields.do_special_countdown = self.special_countdown.0;
        fields.is_asleep = self.is_asleep;
        fields.on_bungee_state = self.grabbed_by_bungee;
        fields.imitater_type = self.imitater_type;
        fields.is_imitation = self.is_imitation;
        fields.target_zombie_id = self.target_zombie;
    }
}

impl BoardObject for Plant {
    fn id(fields: &PlantFields) -> Id<Plant> {
        fields.id
    }

    fn is_dead(fields: &PlantFields) -> bool {
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
        let peashooter = PlantFields {
            id: test_id(1),
            x: 40.0,
            y: 80.0,
            row: 0,
            plant_type: Some(PlantType::Peashooter),
            column: 0,
            state: PlantState::Ready,
            health: 300,
            max_health: 300,
            do_special_countdown: 0,
            state_countdown: 0,
            launch_counter: 150,
            launch_rate: 150,
            on_bungee_state: None,
            imitater_type: None,
            dead: false,
            is_asleep: false,
            is_imitation: false,
            target_zombie_id: None,
            potted_plant_controller: None,
        };
        let chomper = PlantFields {
            id: test_id(2),
            x: 120.0,
            y: 180.0,
            row: 1,
            plant_type: Some(PlantType::Chomper),
            column: 1,
            state: PlantState::ChomperDigesting,
            health: 100,
            max_health: 400,
            do_special_countdown: 5,
            state_countdown: 4000,
            launch_counter: 3,
            launch_rate: 4,
            on_bungee_state: Some(BungeeGrab::Rising),
            imitater_type: Some(PlantType::Imitater),
            dead: true,
            is_asleep: true,
            is_imitation: true,
            target_zombie_id: Some(test_id(5)),
            potted_plant_controller: None,
        };
        assert_round_trips::<Plant>(peashooter, chomper);
    }
}
