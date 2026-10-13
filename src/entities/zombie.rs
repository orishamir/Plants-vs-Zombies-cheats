use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::data_array::BoardObject;
use super::{GameObject, Health, Id, Plant, Position, Ticks};
use crate::il2cpp::{Enum, EnumWithNone, class_layout};

/// A zombie on the lawn.
#[derive(Debug, Clone, PartialEq)]
pub struct Zombie {
    pub zombie_type: ZombieType,
    /// What it's doing, like a Pole Vaulting Zombie in the middle of a vault.
    pub phase: ZombiePhase,
    pub row: i32,
    pub position: Position,
    /// How fast it moves.
    pub speed: f32,
    pub health: Health,
    /// Taking it away here only changes the zombie's data: the zombie still looks like it wears
    /// it, since the game hides headwear when it's knocked off (`Zombie.DropHelm`).
    pub headwear: Option<Armor<HeadwearType>>,
    pub shield: Option<Armor<ShieldType>>,
    /// A Balloon Zombie's balloon, which pops at 0 health.
    pub balloon: Option<Health>,
    /// How long until it's no longer slowed down by a Snow Pea or a Winter Melon, or 0.
    pub chilled: Ticks,
    /// How long until it thaws from an Ice-shroom, or 0. Setting it stops the zombie, but it
    /// keeps its walking animation and isn't drawn in ice: freezing it (`Zombie.HitIceTrap`)
    /// also changes those.
    pub frozen: Ticks,
    /// How long until it shakes off a Kernel-pult's butter, or 0.
    pub buttered: Ticks,
    /// Whether a Hypno-shroom made it fight for the plants. Setting it is enough: the zombie
    /// turns around and is drawn purple.
    pub is_hypnotized: bool,
    pub is_eating: bool,
    /// Zombies drop an arm when they're badly hurt, and their head when they die.
    pub has_head: bool,
    pub has_arm: bool,
    pub is_in_pool: bool,
    /// How long ago it appeared.
    pub age: Ticks,
    /// Which wave it came in.
    pub wave: i32,
    /// The column a Bungee Zombie is dropping into, in its [`row`](Self::row). Dr. Zomboss
    /// sets it too, for the column he's attacking.
    pub target_column: Option<i32>,
    /// The plant a Bungee Zombie grabbed. Until it lands, it's only going for a cell: its
    /// [`target_column`](Self::target_column) in its row.
    pub grabbed_plant: Option<Id<Plant>>,
}

/// Something that protects a zombie until it's destroyed: its headwear or its shield.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Armor<Kind> {
    pub kind: Kind,
    pub health: Health,
}

/// The game's `ZombieType`, with the names players know the zombies by, without "Zombie" at
/// the end: the Conehead Zombie is `ZombieType::Conehead`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ZombieType {
    /// The basic zombie.
    Normal = 0,
    Flag = 1,
    Conehead = 2,     // TrafficCone
    PoleVaulting = 3, // Polevaulter
    Buckethead = 4,   // Pail
    Newspaper = 5,
    ScreenDoor = 6, // Door
    Football = 7,
    Dancing = 8, // Dancer
    BackupDancer = 9,
    DuckyTube = 10,
    Snorkel = 11,
    Zomboni = 12,     // Zamboni
    BobsledTeam = 13, // Bobsled
    DolphinRider = 14,
    JackInTheBox = 15,
    Balloon = 16,
    Digger = 17,
    Pogo = 18,
    Yeti = 19,
    Bungee = 20,
    Ladder = 21,
    Catapult = 22,
    Gargantuar = 23,
    Imp = 24,
    DrZomboss = 25, // Boss
    /// A zombie with a Peashooter for a head, from Zombotany.
    Peashooter = 26, // PeaHead
    /// A zombie with a Wall-nut for a head, from Zombotany.
    WallNut = 27, // WallnutHead
    /// A zombie with a Jalapeno for a head, from Zombotany.
    Jalapeno = 28, // JalapenoHead
    /// A zombie with a Gatling Pea for a head, from Zombotany.
    GatlingPea = 29, // GatlingHead
    /// A zombie with a Squash for a head, from Zombotany.
    Squash = 30, // SquashHead
    /// A zombie with a Tall-nut for a head, from Zombotany.
    TallNut = 31, // TallnutHead
    GigaGargantuar = 32, // RedeyeGargantuar
    /// A zombie made in the Zombatar editor.
    Zombatar = 33,
    Target = 34,
    TrashCan = 35,
    /// A gravestone that zombies come out of, in Versus mode.
    Gravestone = 36,
}

impl Enum for ZombieType {}

/// `ZombieType.Invalid`.
impl EnumWithNone for ZombieType {
    const NONE: i32 = -1;
}

/// What a zombie is doing. Most zombies are only ever [`Normal`](Self::Normal) until they
/// die; the rest of the phases are each for one kind of zombie.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ZombiePhase {
    Normal = 0,  // ZombieNormal
    Dying = 1,   // ZombieDying
    Burned = 2,  // ZombieBurned
    Mowered = 3, // ZombieMowered
    BungeeDiving = 4,
    BungeeDivingScreaming = 5,
    BungeeAtBottom = 6,
    BungeeGrabbing = 7,
    BungeeRising = 8,
    BungeeHitOuchy = 9,
    BungeeCutscene = 10,
    PoleVaulterPreVault = 11,
    PoleVaulterInVault = 12,
    PoleVaulterPostVault = 13,
    RisingFromGrave = 14,
    JackInTheBoxRunning = 15,
    JackInTheBoxPopping = 16,
    BobsledSliding = 17,
    BobsledBoarding = 18,
    BobsledCrashing = 19,
    PogoBouncing = 20,
    PogoHighBounce1 = 21,
    PogoHighBounce2 = 22,
    PogoHighBounce3 = 23,
    PogoHighBounce4 = 24,
    PogoHighBounce5 = 25,
    PogoHighBounce6 = 26,
    PogoForwardBounce2 = 27,
    PogoForwardBounce7 = 28,
    NewspaperReading = 29,
    NewspaperMaddening = 30,
    NewspaperMad = 31,
    DiggerTunneling = 32,
    DiggerRising = 33,
    DiggerTunnelingPauseWithoutAxe = 34,
    DiggerRiseWithoutAxe = 35,
    DiggerStunned = 36,
    DiggerWalking = 37,
    DiggerWalkingWithoutAxe = 38,
    DiggerCutscene = 39,
    DancerDancingIn = 40,
    DancerSnappingFingers = 41,
    DancerSnappingFingersWithLight = 42,
    DancerSnappingFingersHold = 43,
    DancerDancingLeft = 44,
    DancerWalkToRaise = 45,
    DancerRaiseLeft1 = 46,
    DancerRaiseRight1 = 47,
    DancerRaiseLeft2 = 48,
    DancerRaiseRight2 = 49,
    DancerRising = 50,
    DolphinWalking = 51,
    DolphinIntoPool = 52,
    DolphinRiding = 53,
    DolphinInJump = 54,
    DolphinWalkingInPool = 55,
    DolphinWalkingWithoutDolphin = 56,
    SnorkelWalking = 57,
    SnorkelIntoPool = 58,
    SnorkelWalkingInPool = 59,
    SnorkelUpToEat = 60,
    SnorkelEatingInPool = 61,
    SnorkelDownFromEat = 62,
    ZombiquariumAccel = 63,
    ZombiquariumDrift = 64,
    ZombiquariumBackAndForth = 65,
    ZombiquariumBite = 66,
    CatapultLaunching = 67,
    CatapultReloading = 68,
    GargantuarThrowing = 69,
    GargantuarSmashing = 70,
    ImpGettingThrown = 71,
    ImpLanding = 72,
    BalloonFlying = 73,
    BalloonPopping = 74,
    BalloonWalking = 75,
    LadderCarrying = 76,
    LadderPlacing = 77,
    BossEnter = 78,
    BossIdle = 79,
    BossSpawning = 80,
    BossStomping = 81,
    BossBungeesEnter = 82,
    BossBungeesDrop = 83,
    BossBungeesLeave = 84,
    BossDropRV = 85,
    BossHeadEnter = 86,
    BossHeadIdleBeforeSpit = 87,
    BossHeadIdleAfterSpit = 88,
    BossHeadSpit = 89,
    BossHeadLeave = 90,
    YetiRunning = 91,
    SquashPreLaunch = 92,
    SquashRising = 93,
    SquashFalling = 94,
    SquashDoneFalling = 95,
}

impl Enum for ZombiePhase {}

/// What a zombie wears on its head. The game's `HelmType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum HeadwearType {
    Cone = 1,   // TrafficCone
    Bucket = 2, // Pail
    Football = 3,
    /// A Digger Zombie's miner hat.
    HardHat = 4, // Digger
    RedEyes = 5,
    Headband = 6,
    Bobsled = 7,
    /// A Wall-nut Zombie's head.
    WallNut = 8,
    /// A Tall-nut Zombie's head.
    TallNut = 9,
}

impl Enum for HeadwearType {}

/// `HelmType.None`.
impl EnumWithNone for HeadwearType {
    const NONE: i32 = 0;
}

/// What a zombie holds in front of itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ShieldType {
    ScreenDoor = 1, // Door
    Newspaper = 2,
    Ladder = 3,
    Target = 4,
    TrashCan = 5, // Garbage
}

impl Enum for ShieldType {}

/// `ShieldType.None`.
impl EnumWithNone for ShieldType {
    const NONE: i32 = 0;
}

class_layout! {
    /// `Reloaded.Gameplay.Zombie`.
    pub struct ZombieFields = "Reloaded.Gameplay.Zombie" {
        id: Id<Zombie> = "<DataID>k__BackingField",
        row: i32 = "mRow",
        zombie_type: ZombieType = "mZombieType",
        phase: ZombiePhase = "mZombiePhase",
        x: f32 = "mPosX",
        y: f32 = "mPosY",
        velocity_x: f32 = "mVelX",
        is_eating: bool = "mIsEating",
        age: i32 = "mZombieAge",
        from_wave: i32 = "mFromWave",
        chilled_counter: i32 = "mChilledCounter",
        buttered_counter: i32 = "mButteredCounter",
        ice_trap_counter: i32 = "mIceTrapCounter",
        mind_controlled: bool = "mMindControlled",
        has_head: bool = "mHasHead",
        has_arm: bool = "mHasArm",
        in_pool: bool = "mInPool",
        helm_type: Option<HeadwearType> = "mHelmType",
        body_health: i32 = "mBodyHealth",
        body_max_health: i32 = "mBodyMaxHealth",
        helm_health: i32 = "mHelmHealth",
        helm_max_health: i32 = "mHelmMaxHealth",
        shield_type: Option<ShieldType> = "mShieldType",
        shield_health: i32 = "mShieldHealth",
        shield_max_health: i32 = "mShieldMaxHealth",
        flying_health: i32 = "mFlyingHealth",
        flying_max_health: i32 = "mFlyingMaxHealth",
        dead: bool = "mDead",
        /// -1 for none.
        target_col: i32 = "mTargetCol",
        target_plant_id: Option<Id<Plant>> = "mTargetPlantID",
    }
}

impl GameObject for Zombie {
    type Fields = ZombieFields;

    fn from_fields(fields: &ZombieFields) -> Option<Self> {
        Some(Self {
            zombie_type: fields.zombie_type,
            phase: fields.phase,
            row: fields.row,
            position: Position {
                x: fields.x,
                y: fields.y,
            },
            speed: fields.velocity_x,
            health: Health {
                current: fields.body_health,
                max: fields.body_max_health,
            },
            headwear: fields.helm_type.map(|kind| Armor {
                kind,
                health: Health {
                    current: fields.helm_health,
                    max: fields.helm_max_health,
                },
            }),
            shield: fields.shield_type.map(|kind| Armor {
                kind,
                health: Health {
                    current: fields.shield_health,
                    max: fields.shield_max_health,
                },
            }),
            // Only Balloon Zombies have any.
            balloon: (fields.flying_max_health > 0).then_some(Health {
                current: fields.flying_health,
                max: fields.flying_max_health,
            }),
            chilled: Ticks(fields.chilled_counter),
            frozen: Ticks(fields.ice_trap_counter),
            buttered: Ticks(fields.buttered_counter),
            is_hypnotized: fields.mind_controlled,
            is_eating: fields.is_eating,
            has_head: fields.has_head,
            has_arm: fields.has_arm,
            is_in_pool: fields.in_pool,
            age: Ticks(fields.age),
            wave: fields.from_wave,
            target_column: (fields.target_col >= 0).then_some(fields.target_col),
            grabbed_plant: fields.target_plant_id,
        })
    }

    fn update_fields(&self, fields: &mut ZombieFields) {
        fields.zombie_type = self.zombie_type;
        fields.phase = self.phase;
        fields.row = self.row;
        fields.x = self.position.x;
        fields.y = self.position.y;
        fields.velocity_x = self.speed;
        fields.body_health = self.health.current;
        fields.body_max_health = self.health.max;

        fields.helm_type = self.headwear.map(|headwear| headwear.kind);
        if let Some(headwear) = self.headwear {
            fields.helm_health = headwear.health.current;
            fields.helm_max_health = headwear.health.max;
        }
        fields.shield_type = self.shield.map(|shield| shield.kind);
        if let Some(shield) = self.shield {
            fields.shield_health = shield.health.current;
            fields.shield_max_health = shield.health.max;
        }
        let balloon = self.balloon.unwrap_or(Health { current: 0, max: 0 });
        fields.flying_health = balloon.current;
        fields.flying_max_health = balloon.max;

        fields.chilled_counter = self.chilled.0;
        fields.ice_trap_counter = self.frozen.0;
        fields.buttered_counter = self.buttered.0;
        fields.mind_controlled = self.is_hypnotized;
        fields.is_eating = self.is_eating;
        fields.has_head = self.has_head;
        fields.has_arm = self.has_arm;
        fields.in_pool = self.is_in_pool;
        fields.age = self.age.0;
        fields.from_wave = self.wave;
        fields.target_col = self.target_column.unwrap_or(-1);
        fields.target_plant_id = self.grabbed_plant;
    }
}

impl BoardObject for Zombie {
    fn id(fields: &ZombieFields) -> Id<Zombie> {
        fields.id
    }

    fn is_dead(fields: &ZombieFields) -> bool {
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
        let zombie = ZombieFields {
            id: test_id(1),
            row: 0,
            zombie_type: ZombieType::Normal,
            phase: ZombiePhase::Normal,
            x: 800.0,
            y: 60.0,
            velocity_x: 0.2,
            is_eating: false,
            age: 100,
            from_wave: 1,
            chilled_counter: 0,
            buttered_counter: 0,
            ice_trap_counter: 0,
            mind_controlled: false,
            has_head: true,
            has_arm: true,
            in_pool: false,
            helm_type: None,
            body_health: 270,
            body_max_health: 270,
            helm_health: 0,
            helm_max_health: 0,
            shield_type: None,
            shield_health: 0,
            shield_max_health: 0,
            flying_health: 0,
            flying_max_health: 0,
            dead: false,
            target_col: -1,
            target_plant_id: None,
        };
        let balloon_zombie = ZombieFields {
            id: test_id(2),
            row: 2,
            zombie_type: ZombieType::Balloon,
            phase: ZombiePhase::BalloonFlying,
            x: 500.0,
            y: 260.0,
            velocity_x: 0.4,
            is_eating: true,
            age: 900,
            from_wave: 4,
            chilled_counter: 30,
            buttered_counter: 40,
            ice_trap_counter: 50,
            mind_controlled: true,
            has_head: false,
            has_arm: false,
            in_pool: true,
            helm_type: Some(HeadwearType::Bucket),
            body_health: 100,
            body_max_health: 200,
            helm_health: 1000,
            helm_max_health: 1100,
            shield_type: Some(ShieldType::ScreenDoor),
            shield_health: 600,
            shield_max_health: 1100,
            flying_health: 10,
            flying_max_health: 20,
            dead: true,
            target_col: 4,
            target_plant_id: Some(test_id(3)),
        };
        assert_round_trips::<Zombie>(zombie, balloon_zombie);
    }
}
