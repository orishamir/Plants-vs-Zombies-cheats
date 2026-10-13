use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::challenge::ChallengeFields;
use super::coin::CoinFields;
use super::data_array::{DataArray, read_living};
use super::grid_item::GridItemFields;
use super::lawn_mower::LawnMowerFields;
use super::level::{LevelEntryDataFields, LevelInfo};
use super::plant::PlantFields;
use super::projectile::ProjectileFields;
use super::seed_packet::SeedBankFields;
use super::zombie::ZombieFields;
use super::{
    Challenge, Coin, Entity, GameObject, GridItem, LawnMower, Plant, Projectile, SeedPacket, Ticks,
    Zombie,
};
use crate::il2cpp::{
    Array, Class, CsString, DecodeError, Element, Enum, List, Object, ObjectError, Ref, TypeInfo,
    class_layout, read_struct,
};
use crate::{Dll, Popcapgame};

/// The lawn during a level.
///
/// Get it with [`Board::current`], and what's on it with the methods of `Entity<Board>`,
/// like [`zombies`](Entity::zombies).
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub background: Background,
    /// The level's number in Adventure mode, counting through the worlds: level 1-2 is 2, and
    /// 2-1 is 11. It's 0 in the other modes, like puzzles and mini-games.
    pub level: i32,
    /// The wave of zombies the level is at, out of [`waves`](Self::waves).
    pub wave: i32,
    pub waves: i32,
    pub is_paused: bool,
    /// How long until the next sun falls from the sky.
    pub next_sun_in: Ticks,
    /// How long until the next wave of zombies comes.
    pub next_wave_in: Ticks,
    /// How many times the board has updated, which is how long the level has been going.
    pub time: Ticks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Background {
    Day = 0,
    Night = 1,
    Pool = 2,
    Fog = 3,
    Roof = 4,
    /// The roof at night, where Dr. Zomboss is fought.
    Boss = 5,
    MushroomGarden = 6,
    Greenhouse = 7,
    Zombiquarium = 8,
    TreeOfWisdom = 9,
    China = 10,
}

impl Enum for Background {}

/// The sun a player has to spend on plants.
#[derive(Debug, Clone, PartialEq)]
pub struct Sun {
    pub amount: i32,
}

/// `Tekly.DataModels.Models.RootModel`, whose static `Instance` holds the game's data models,
/// which its UI shows. Reading the board starts here.
pub const ROOT_MODEL: TypeInfo = TypeInfo {
    name: "Tekly.DataModels.Models.RootModel_TypeInfo",
    location: Dll::GameAssembly.at(0x4C56028),
};

/// The key of the board's data model, a `GameplayDataModel`, in the root model. This is how
/// the game finds it too, in `Source.Utils.DataModelUtils$$UpdateGameplayBoard`.
const GAMEPLAY_MODEL_KEY: &str = "gameplay";

impl Board {
    /// The board of the level being played, or `None` when there isn't one.
    pub fn current(game: &Popcapgame) -> Result<Option<Entity<Board>>, ObjectError> {
        let root: Ref<RootModel> = ROOT_MODEL
            .class(game)?
            .get_static(game, "<Instance>k__BackingField")?;
        for model in root.read(game)?.models.read(game)?.elements(game)? {
            let (Some(key), Some(model)) = (model.key, model.model) else {
                continue;
            };
            if key.read_string(game)? == GAMEPLAY_MODEL_KEY {
                let gameplay = Ref::<GameplayDataModel>::from(model).read(game)?;
                return gameplay
                    .board
                    .map(|board| Entity::read(game, board))
                    .transpose();
            }
        }
        Ok(None)
    }
}

impl Entity<Board> {
    pub fn zombies(&self, game: &Popcapgame) -> Result<Vec<Entity<Zombie>>, ObjectError> {
        read_living(game, self.fields.zombies)
    }

    pub fn plants(&self, game: &Popcapgame) -> Result<Vec<Entity<Plant>>, ObjectError> {
        read_living(game, self.fields.plants)
    }

    pub fn projectiles(&self, game: &Popcapgame) -> Result<Vec<Entity<Projectile>>, ObjectError> {
        read_living(game, self.fields.projectiles)
    }

    pub fn coins(&self, game: &Popcapgame) -> Result<Vec<Entity<Coin>>, ObjectError> {
        read_living(game, self.fields.coins)
    }

    pub fn lawn_mowers(&self, game: &Popcapgame) -> Result<Vec<Entity<LawnMower>>, ObjectError> {
        read_living(game, self.fields.lawn_mowers)
    }

    pub fn grid_items(&self, game: &Popcapgame) -> Result<Vec<Entity<GridItem>>, ObjectError> {
        read_living(game, self.fields.grid_items)
    }

    /// The seed packets in the seed bank, from left to right.
    pub fn seed_packets(&self, game: &Popcapgame) -> Result<Vec<Entity<SeedPacket>>, ObjectError> {
        let Some(seed_bank) = self.fields.seed_bank else {
            return Ok(Vec::new());
        };
        let seed_bank = seed_bank.read(game)?;
        let count = usize::try_from(seed_bank.packet_count).unwrap_or_default();
        seed_bank
            .packets
            .read_first(game, count)?
            .into_iter()
            .flatten()
            .map(|packet| Entity::read(game, packet))
            .collect()
    }

    /// What the level is and how it's set up, like its mode.
    pub fn level_info(&self, game: &Popcapgame) -> Result<Option<LevelInfo>, ObjectError> {
        self.fields
            .level_entry_data
            .map(|data| LevelInfo::read(game, data))
            .transpose()
    }

    /// What the level's mode keeps track of, like Beghouled's craters.
    pub fn challenge(&self, game: &Popcapgame) -> Result<Option<Entity<Challenge>>, ObjectError> {
        self.fields
            .challenge
            .map(|challenge| Entity::read(game, challenge))
            .transpose()
    }

    /// The first player's sun.
    pub fn sun(&self, game: &Popcapgame) -> Result<Entity<Sun>, ObjectError> {
        let players = self.fields.sun_money.read(game)?.values;
        let first = players
            .read_first(game, 1)?
            .into_iter()
            .flatten()
            .next()
            .ok_or_else(|| ObjectError::InvalidValue {
                place: "the first player's Reloaded.Gameplay.Board.mSunMoney".to_owned(),
                error: DecodeError::Null,
            })?;
        Entity::read(game, first)
    }
}

class_layout! {
    /// `Tekly.DataModels.Models.RootModel`, an `ObjectModel`: a list of models, each with a
    /// key.
    pub struct RootModel = "Tekly.DataModels.Models.RootModel" {
        models: Ref<List<ModelReference>> = "m_models",
    }
}

class_layout! {
    /// `Tekly.DataModels.Models.ModelReference`, a struct.
    pub struct ModelReference = "Tekly.DataModels.Models.ModelReference" {
        model: Option<Object> = "Model",
        key: Option<Ref<CsString>> = "Key",
    }
}

impl Element for ModelReference {
    fn read_element(game: &Popcapgame, class: Class, bytes: &[u8]) -> Result<Self, ObjectError> {
        read_struct(game, class, bytes)
    }
}

class_layout! {
    /// `Reloaded.DataModels.GameplayDataModel`, what the UI shows during a level.
    pub struct GameplayDataModel = "Reloaded.DataModels.GameplayDataModel" {
        board: Option<Ref<BoardFields>> = "m_board",
    }
}

class_layout! {
    /// `Reloaded.Gameplay.Board`.
    pub struct BoardFields = "Reloaded.Gameplay.Board" {
        zombies: Ref<DataArray<ZombieFields>> = "m_zombies",
        plants: Ref<DataArray<PlantFields>> = "m_plants",
        projectiles: Ref<DataArray<ProjectileFields>> = "m_projectiles",
        coins: Ref<DataArray<CoinFields>> = "m_coins",
        lawn_mowers: Ref<DataArray<LawnMowerFields>> = "m_lawnMowers",
        grid_items: Ref<DataArray<GridItemFields>> = "m_gridItems",
        seed_bank: Option<Ref<SeedBankFields>> = "mSeedBank",
        challenge: Option<Ref<ChallengeFields>> = "mChallenge",
        paused: bool = "mPaused",
        sun_count_down: i32 = "mSunCountDown",
        background: Background = "mBackground",
        level: i32 = "mLevel",
        level_entry_data: Option<Ref<LevelEntryDataFields>> = "mLevelEntryData",
        sun_money: Ref<MultiplayerType<Option<Ref<SunFields>>>> = "mSunMoney",
        num_waves: i32 = "mNumWaves",
        main_counter: i32 = "mMainCounter",
        current_wave: i32 = "mCurrentWave",
        zombie_count_down: i32 = "mZombieCountDown",
    }
}

class_layout! {
    /// `Reloaded.Utils.MultiplayerType<T>`, a value for each player.
    pub struct MultiplayerType<T> = "Reloaded.Utils.MultiplayerType" {
        values: Ref<Array<T>> = "m_values",
    }
}

class_layout! {
    /// `Reloaded.Gameplay.Sun`.
    pub struct SunFields = "Reloaded.Gameplay.Sun" {
        amount: i32 = "Amount",
    }
}

impl GameObject for Board {
    type Fields = BoardFields;

    fn from_fields(fields: &BoardFields) -> Option<Self> {
        Some(Self {
            background: fields.background,
            level: fields.level,
            wave: fields.current_wave,
            waves: fields.num_waves,
            is_paused: fields.paused,
            next_sun_in: Ticks(fields.sun_count_down),
            next_wave_in: Ticks(fields.zombie_count_down),
            time: Ticks(fields.main_counter),
        })
    }

    fn update_fields(&self, fields: &mut BoardFields) {
        fields.background = self.background;
        fields.level = self.level;
        fields.current_wave = self.wave;
        fields.num_waves = self.waves;
        fields.paused = self.is_paused;
        fields.sun_count_down = self.next_sun_in.0;
        fields.zombie_count_down = self.next_wave_in.0;
        fields.main_counter = self.time.0;
    }
}

impl GameObject for Sun {
    type Fields = SunFields;

    fn from_fields(fields: &SunFields) -> Option<Self> {
        Some(Self {
            amount: fields.amount,
        })
    }

    fn update_fields(&self, fields: &mut SunFields) {
        fields.amount = self.amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::entity::assert_round_trips;

    /// A reference to nothing in particular, which the test never reads.
    fn any<T>(address: usize) -> Ref<T> {
        Ref::from(Object::at(address).unwrap())
    }

    fn board(level: i32) -> BoardFields {
        BoardFields {
            zombies: any(0x100),
            plants: any(0x200),
            projectiles: any(0x300),
            coins: any(0x400),
            lawn_mowers: any(0x500),
            grid_items: any(0x600),
            seed_bank: None,
            challenge: None,
            paused: false,
            sun_count_down: 100,
            background: Background::Day,
            level,
            level_entry_data: None,
            sun_money: any(0x700),
            num_waves: 2,
            main_counter: 1000,
            current_wave: 1,
            zombie_count_down: 500,
        }
    }

    #[test]
    fn round_trips() {
        let other = BoardFields {
            paused: true,
            sun_count_down: 1,
            background: Background::Roof,
            num_waves: 3,
            main_counter: 2,
            current_wave: 3,
            zombie_count_down: 4,
            ..board(42)
        };
        assert_round_trips::<Board>(board(1), other);
        assert_round_trips::<Sun>(SunFields { amount: 50 }, SunFields { amount: 9990 });
    }
}
