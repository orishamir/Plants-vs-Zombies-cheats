use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::ZombieType;
use crate::Game;
use crate::il2cpp::{Array, CsString, Enum, EnumWithNone, ObjectError, Ref, class_layout};

/// What the level is and how it's set up. The game's `LevelEntryData`, which describes a level
/// before it starts; read it with [`level_info`](super::Entity::level_info).
#[derive(Debug, Clone, PartialEq)]
pub struct LevelInfo {
    /// The level's name in the game's data, like `$SURVIVAL_NIGHT_HARD`.
    pub name: String,
    pub mode: GameMode,
    /// For the modes Replanted added, which one.
    pub reloaded_mode: Option<ReloadedGameMode>,
    /// Which menu the level is in, like the mini-games or the puzzles.
    pub category: Option<LevelCategory>,
    /// Where the level takes place.
    pub area: Option<GameArea>,
    pub number: i32,
    /// How many waves of zombies the level has.
    pub waves: i32,
    /// How much sun the level starts with.
    pub initial_sun: i32,
    /// The kinds of zombies the level's data lists. They aren't always all the zombies that
    /// come: Survival: Night (Hard) only lists the basic Zombie.
    pub zombie_types: Vec<ZombieType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum GameMode {
    Adventure = 0,
    SurvivalNormalStage1 = 1,
    SurvivalNormalStage2 = 2,
    SurvivalNormalStage3 = 3,
    SurvivalNormalStage4 = 4,
    SurvivalNormalStage5 = 5,
    SurvivalHardStage1 = 6,
    SurvivalHardStage2 = 7,
    SurvivalHardStage3 = 8,
    SurvivalHardStage4 = 9,
    SurvivalHardStage5 = 10,
    SurvivalEndlessStage1 = 11,
    SurvivalEndlessStage2 = 12,
    SurvivalEndlessStage3 = 13,
    SurvivalEndlessStage4 = 14,
    SurvivalEndlessStage5 = 15,
    ChallengeWarAndPeas = 16,
    ChallengeWallnutBowling = 17,
    ChallengeSlotMachine = 18,
    ChallengeRainingSeeds = 19,
    ChallengeBeghouled = 20,
    ChallengeInvisighoul = 21,
    ChallengeSeeingStars = 22,
    ChallengeZombiquarium = 23,
    ChallengeBeghouledTwist = 24,
    ChallengeLittleTrouble = 25,
    ChallengePortalCombat = 26,
    ChallengeColumn = 27,
    ChallengeBobsledBonanza = 28,
    ChallengeSpeed = 29,
    ChallengeWhackAZombie = 30,
    ChallengeLastStand = 31,
    ChallengeWarAndPeas2 = 32,
    ChallengeWallnutBowling2 = 33,
    ChallengePogoParty = 34,
    ChallengeFinalBoss = 35,
    ChallengeArtChallenge1 = 36,
    ChallengeSunnyDay = 37,
    ChallengeResodded = 38,
    ChallengeBigTime = 39,
    ChallengeArtChallenge2 = 40,
    ChallengeAirRaid = 41,
    ChallengeIce = 42,
    ChallengeZenGarden = 43,
    ChallengeHighGravity = 44,
    ChallengeGraveDanger = 45,
    ChallengeShovel = 46,
    ChallengeStormyNight = 47,
    ChallengeBungeeBlitz = 48,
    ChallengeSquirrel = 49,
    TreeOfWisdom = 50,
    ScaryPotter1 = 51,
    ScaryPotter2 = 52,
    ScaryPotter3 = 53,
    ScaryPotter4 = 54,
    ScaryPotter5 = 55,
    ScaryPotter6 = 56,
    ScaryPotter7 = 57,
    ScaryPotter8 = 58,
    ScaryPotter9 = 59,
    ScaryPotterEndless = 60,
    PuzzleIZombie1 = 61,
    PuzzleIZombie2 = 62,
    PuzzleIZombie3 = 63,
    PuzzleIZombie4 = 64,
    PuzzleIZombie5 = 65,
    PuzzleIZombie6 = 66,
    PuzzleIZombie7 = 67,
    PuzzleIZombie8 = 68,
    PuzzleIZombie9 = 69,
    PuzzleIZombieEndless = 70,
    Upsell = 71,
    Intro = 72,
    BonusChina = 73,
    Coop = 74,
    Versus = 75,
}

impl Enum for GameMode {}

/// The game's `ReloadedGameMode`: the modes Replanted added.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ReloadedGameMode {
    CloudyDay = 0,
    RestInPeace = 1,
    Cooperative = 2,
    Versus = 3,
    China = 4,
}

impl Enum for ReloadedGameMode {}

/// `ReloadedGameMode.None`.
impl EnumWithNone for ReloadedGameMode {
    const NONE: i32 = -1;
}

/// Which menu a level is in. The game's `ChallengeEntryType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum LevelCategory {
    MiniGame = 1,
    Puzzle = 2,
    Survival = 3,
    Coop = 4,
    Versus = 5,
    Limbo = 6,
}

impl Enum for LevelCategory {}

/// `ChallengeEntryType.None`.
impl EnumWithNone for LevelCategory {
    const NONE: i32 = 0;
}

/// Where a level takes place.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum GameArea {
    Day = 0,
    Night = 1,
    Pool = 2,
    Fog = 3,
    Roof = 4,
    /// The roof at night, where Dr. Zomboss is fought.
    Boss = 5,
    ZenGarden = 6,
    TreeOfWisdom = 7,
    China = 8,
}

impl Enum for GameArea {}

/// `GameArea.Invalid`.
impl EnumWithNone for GameArea {
    const NONE: i32 = -1;
}

class_layout! {
    /// `Reloaded.Data.LevelEntryData`.
    pub struct LevelEntryDataFields = "Reloaded.Data.LevelEntryData" {
        level_name: Option<Ref<CsString>> = "m_levelName",
        game_mode: GameMode = "m_gameMode",
        reloaded_game_mode: Option<ReloadedGameMode> = "m_reloadedGameMode",
        level_number: i32 = "m_levelNumber",
        game_area: Option<GameArea> = "m_gameArea",
        zombie_types: Option<Ref<Array<ZombieType>>> = "m_zombieTypes",
        zombie_waves: i32 = "m_zombieWaves",
        initial_sun: i32 = "m_initialSun",
        entry_type: Option<LevelCategory> = "m_entryType",
    }
}

impl LevelInfo {
    /// Reads the level's data, with its name and zombies, which are separate objects.
    pub(super) fn read(game: &Game, data: Ref<LevelEntryDataFields>) -> Result<Self, ObjectError> {
        let fields = data.read(game)?;
        Ok(Self {
            name: match fields.level_name {
                Some(name) => name.read_string(game)?,
                None => String::new(),
            },
            mode: fields.game_mode,
            reloaded_mode: fields.reloaded_game_mode,
            category: fields.entry_type,
            area: fields.game_area,
            number: fields.level_number,
            waves: fields.zombie_waves,
            initial_sun: fields.initial_sun,
            zombie_types: match fields.zombie_types {
                Some(zombie_types) => zombie_types.read_all(game)?,
                None => Vec::new(),
            },
        })
    }
}
