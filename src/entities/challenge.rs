use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::{Cell, Entity, GameObject};
use crate::Popcapgame;
use crate::il2cpp::{Array, Array2D, Enum, ObjectError, Ref, class_layout};

/// What the level's mini-game, puzzle or other mode keeps track of, like Beghouled's craters.
/// The game's `Challenge`.
///
/// Its arrays are read separately: see [`beghouled_craters`](Entity::beghouled_craters) and
/// [`beghouled_upgrades`](Entity::beghouled_upgrades).
#[derive(Debug, Clone, PartialEq)]
pub struct Challenge {
    pub state: ChallengeState,
    /// A counter that each mode uses its own way. In Portal Combat, it counts down to when a
    /// portal moves (`Challenge.UpdatePortalCombat`).
    pub state_counter: i32,
    /// The score, in the modes that keep one, like Beghouled and I, Zombie.
    pub score: i32,
    /// How many matches Beghouled's last move made.
    pub beghouled_matches: i32,
}

/// What the level's mode is doing. Most of the states are each for one mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ChallengeState {
    Normal = 0,
    BeghouledMoving = 1,
    BeghouledFalling = 2,
    BeghouledNoMatches = 3,
    SlotMachineRolling = 4,
    StormFlash1 = 5,
    StormFlash2 = 6,
    StormFlash3 = 7,
    ZenFading = 8,
    ScaryPotterMalleting = 9,
    LastStandOnslaught = 10,
    TreeJustGrew = 11,
    TreeGiveWisdom = 12,
    TreeWaitingToBabble = 13,
    TreeBabbling = 14,
}

impl Enum for ChallengeState {}

/// What Beghouled's upgrades turn a plant into. The game's `BeghouledUpgrade`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum BeghouledUpgrade {
    /// Peashooters into Repeaters.
    Repeater = 0,
    /// Puff-shrooms into Fume-shrooms.
    FumeShroom = 1, // Fumeshroom
    /// Wall-nuts into Tall-nuts.
    TallNut = 2, // Tallnut
}

impl Entity<Challenge> {
    /// Beghouled's craters: the cells where a zombie ate a plant, which nothing can be moved
    /// into until the crater is filled.
    pub fn beghouled_craters(&self, game: &Popcapgame) -> Result<Vec<Cell>, ObjectError> {
        let Some(eaten) = self.fields.beghouled_eaten else {
            return Ok(Vec::new());
        };
        // `new bool[9, 6]` in `Challenge..ctor`: the columns, then the rows.
        let mut craters = Vec::new();
        for (column, rows) in eaten.read_all(game)?.into_iter().enumerate() {
            for (row, is_crater) in rows.into_iter().enumerate() {
                if is_crater {
                    craters.push(Cell {
                        row: row as i32,
                        column: column as i32,
                    });
                }
            }
        }
        Ok(craters)
    }

    /// The Beghouled upgrades that have been bought.
    pub fn beghouled_upgrades(
        &self,
        game: &Popcapgame,
    ) -> Result<Vec<BeghouledUpgrade>, ObjectError> {
        let Some(purchased) = self.fields.beghouled_purchased_upgrade else {
            return Ok(Vec::new());
        };
        // One for each upgrade, in `BeghouledUpgrade`'s order.
        Ok(purchased
            .read_all(game)?
            .into_iter()
            .enumerate()
            .filter(|&(_, is_purchased)| is_purchased)
            .filter_map(|(upgrade, _)| BeghouledUpgrade::try_from(upgrade as i32).ok())
            .collect())
    }
}

class_layout! {
    /// `Reloaded.Gameplay.Challenge`.
    pub struct ChallengeFields = "Reloaded.Gameplay.Challenge" {
        beghouled_eaten: Option<Ref<Array2D<bool>>> = "mBeghouledEated",
        beghouled_purchased_upgrade: Option<Ref<Array<bool>>> = "mBeghouledPurcasedUpgrade",
        beghouled_matches_this_move: i32 = "mBeghouledMatchesThisMove",
        state: ChallengeState = "mChallengeState",
        state_counter: i32 = "mChallengeStateCounter",
        score: i32 = "mChallengeScore",
    }
}

impl GameObject for Challenge {
    type Fields = ChallengeFields;

    fn from_fields(fields: &ChallengeFields) -> Option<Self> {
        Some(Self {
            state: fields.state,
            state_counter: fields.state_counter,
            score: fields.score,
            beghouled_matches: fields.beghouled_matches_this_move,
        })
    }

    fn update_fields(&self, fields: &mut ChallengeFields) {
        fields.state = self.state;
        fields.state_counter = self.state_counter;
        fields.score = self.score;
        fields.beghouled_matches_this_move = self.beghouled_matches;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::entity::assert_round_trips;

    #[test]
    fn round_trips() {
        let portal_combat = ChallengeFields {
            beghouled_eaten: None,
            beghouled_purchased_upgrade: None,
            beghouled_matches_this_move: 0,
            state: ChallengeState::Normal,
            state_counter: 9000,
            score: 0,
        };
        let beghouled = ChallengeFields {
            beghouled_matches_this_move: 2,
            state: ChallengeState::BeghouledFalling,
            state_counter: 10,
            score: 40,
            ..portal_combat.clone()
        };
        assert_round_trips::<Challenge>(portal_combat, beghouled);
    }
}
