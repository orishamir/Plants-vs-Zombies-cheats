use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::data_array::BoardObject;
use super::{GameObject, Id, Position};
use crate::il2cpp::{Enum, class_layout};

/// A lawn mower, or one of the other machines at the left end of a row that clear it once.
#[derive(Debug, Clone, PartialEq)]
pub struct LawnMower {
    pub mower_type: LawnMowerType,
    pub state: LawnMowerState,
    pub row: i32,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum LawnMowerType {
    Normal = 0,      // Lawn
    PoolCleaner = 1, // Pool
    RoofCleaner = 2, // Roof
    SuperMower = 3,
}

impl Enum for LawnMowerType {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum LawnMowerState {
    /// Rolling onto the lawn when the level starts.
    RollingIn = 0,
    Ready = 1,
    /// Mowing down its row.
    Triggered = 2,
    Squished = 3,
}

impl Enum for LawnMowerState {}

class_layout! {
    /// `Reloaded.Gameplay.LawnMower`.
    pub struct LawnMowerFields = "Reloaded.Gameplay.LawnMower" {
        id: Id<LawnMower> = "<DataID>k__BackingField",
        x: f32 = "mX",
        y: f32 = "mY",
        row: i32 = "mRow",
        state: LawnMowerState = "mMowerState",
        dead: bool = "mDead",
        mower_type: LawnMowerType = "mMowerType",
    }
}

impl GameObject for LawnMower {
    type Fields = LawnMowerFields;

    fn from_fields(fields: &LawnMowerFields) -> Option<Self> {
        Some(Self {
            mower_type: fields.mower_type,
            state: fields.state,
            row: fields.row,
            position: Position {
                x: fields.x,
                y: fields.y,
            },
        })
    }

    fn update_fields(&self, fields: &mut LawnMowerFields) {
        fields.mower_type = self.mower_type;
        fields.state = self.state;
        fields.row = self.row;
        fields.x = self.position.x;
        fields.y = self.position.y;
    }
}

impl BoardObject for LawnMower {
    fn id(fields: &LawnMowerFields) -> Id<LawnMower> {
        fields.id
    }

    fn is_dead(fields: &LawnMowerFields) -> bool {
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
        let mower = LawnMowerFields {
            id: test_id(1),
            x: -20.0,
            y: 80.0,
            row: 0,
            state: LawnMowerState::Ready,
            dead: false,
            mower_type: LawnMowerType::Normal,
        };
        let pool_cleaner = LawnMowerFields {
            id: test_id(2),
            x: 300.0,
            y: 280.0,
            row: 2,
            state: LawnMowerState::Triggered,
            dead: true,
            mower_type: LawnMowerType::PoolCleaner,
        };
        assert_round_trips::<LawnMower>(mower, pool_cleaner);
    }
}
