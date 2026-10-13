use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::data_array::BoardObject;
use super::{GameObject, Id, Position, Ticks, Zombie};
use crate::il2cpp::{Enum, class_layout};

/// Something a plant shot or threw, like a pea or a cabbage.
#[derive(Debug, Clone, PartialEq)]
pub struct Projectile {
    pub projectile_type: ProjectileType,
    /// How it flies.
    pub motion: ProjectileMotion,
    pub row: i32,
    pub position: Position,
    /// How long ago it was shot.
    pub age: Ticks,
    /// The zombie a Cattail's spike is homing in on.
    pub target_zombie: Option<Id<Zombie>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ProjectileType {
    Pea = 0,
    FrozenPea = 1, // Snowpea
    Cabbage = 2,
    Watermelon = 3, // Melon
    /// A Puff-shroom's, Sea-shroom's or Scaredy-shroom's spore.
    Puff = 4,
    WinterMelon = 5,
    /// A pea that went through a Torchwood.
    FlamingPea = 6, // Fireball
    Star = 7,
    /// A Cactus's or a Cattail's spike.
    Spike = 8,
    /// What a Catapult Zombie throws.
    Basketball = 9,
    Kernel = 10,
    /// A Cob Cannon's cob.
    Cob = 11, // Cobbig
    Butter = 12,
    /// A Peashooter Zombie's pea.
    ZombiePea = 13,
    /// A Peashooter's pea. Other plants, like the Threepeater, shoot [`Pea`](Self::Pea)s.
    PeashooterPea = 14,
    PeashooterFireball = 15,
}

impl Enum for ProjectileType {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ProjectileMotion {
    Straight = 0,
    /// Thrown in an arc, like a Cabbage-pult's cabbage.
    Lobbed = 1,
    /// A Threepeater's two side peas, which move to the row above or below. Its middle pea
    /// flies [`Straight`](Self::Straight).
    Threepeater = 2,
    Bee = 3,
    BeeBackwards = 4,
    Puff = 5,
    /// Flying left, like a Peashooter Zombie's pea.
    Backwards = 6,
    Star = 7,
    FloatOver = 8,
    /// A Cattail's spike, which follows its target.
    Homing = 9,
}

impl Enum for ProjectileMotion {}

class_layout! {
    /// `Reloaded.Gameplay.Projectile`.
    pub struct ProjectileFields = "Reloaded.Gameplay.Projectile" {
        id: Id<Projectile> = "<DataID>k__BackingField",
        row: i32 = "mRow",
        x: f32 = "mPosX",
        y: f32 = "mPosY",
        dead: bool = "mDead",
        motion: ProjectileMotion = "mMotionType",
        projectile_type: ProjectileType = "mProjectileType",
        age: i32 = "mProjectileAge",
        /// Only a Cattail's spikes have one (`Plant.Fire`).
        target_zombie_id: Option<Id<Zombie>> = "mTargetZombieID",
    }
}

impl GameObject for Projectile {
    type Fields = ProjectileFields;

    fn from_fields(fields: &ProjectileFields) -> Option<Self> {
        Some(Self {
            projectile_type: fields.projectile_type,
            motion: fields.motion,
            row: fields.row,
            position: Position {
                x: fields.x,
                y: fields.y,
            },
            age: Ticks(fields.age),
            target_zombie: fields.target_zombie_id,
        })
    }

    fn update_fields(&self, fields: &mut ProjectileFields) {
        fields.projectile_type = self.projectile_type;
        fields.motion = self.motion;
        fields.row = self.row;
        fields.x = self.position.x;
        fields.y = self.position.y;
        fields.age = self.age.0;
        fields.target_zombie_id = self.target_zombie;
    }
}

impl BoardObject for Projectile {
    fn id(fields: &ProjectileFields) -> Id<Projectile> {
        fields.id
    }

    fn is_dead(fields: &ProjectileFields) -> bool {
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
        let pea = ProjectileFields {
            id: test_id(1),
            row: 0,
            x: 100.0,
            y: 60.0,
            dead: false,
            motion: ProjectileMotion::Straight,
            projectile_type: ProjectileType::Pea,
            age: 5,
            target_zombie_id: None,
        };
        let cabbage = ProjectileFields {
            id: test_id(2),
            row: 3,
            x: 200.0,
            y: 360.0,
            dead: true,
            motion: ProjectileMotion::Lobbed,
            projectile_type: ProjectileType::Cabbage,
            age: 50,
            target_zombie_id: Some(test_id(9)),
        };
        assert_round_trips::<Projectile>(pea, cabbage);
    }
}
