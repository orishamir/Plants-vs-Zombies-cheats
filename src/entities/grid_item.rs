use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::data_array::BoardObject;
use super::{Cell, GameObject, Id, PlantType, Position, Ticks, ZombieType};
use crate::il2cpp::{Enum, EnumWithNone, class_layout};

/// Something on the lawn's grid that isn't a plant, like a gravestone, a crater or a vase.
#[derive(Debug, Clone, PartialEq)]
pub struct GridItem {
    pub kind: GridItemKind,
    pub cell: Cell,
    /// Where it's drawn. Only some kinds of items have one, like the brains in I, Zombie;
    /// others, like gravestones and vases, are at (0, 0).
    pub position: Position,
}

/// What a [`GridItem`] is, with what's only stored for that kind of item.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridItemKind {
    Gravestone {
        /// How far it has risen out of the ground, up to 100.
        risen: i32,
        /// The game picks one of a level's gravestones to be special
        /// (`Board.PickSpecialGraveStone`).
        is_special: bool,
    },
    /// The hole a Doom-shroom leaves, which nothing can be planted in until it's gone.
    Crater {
        disappears_in: Ticks,
    },
    /// A ladder that a Ladder Zombie put on a plant.
    Ladder,
    /// A portal in Portal Combat. What goes into a portal comes out of the other one with the
    /// same shape (`Challenge.GetOtherPortal`).
    Portal {
        shape: PortalShape,
        /// Closing, before the game opens it somewhere else (`GridItem.ClosePortal`).
        is_closing: bool,
    },
    /// A brain dropped in Zombiquarium.
    Brain,
    /// A vase in Vasebreaker. The game calls it a scary pot.
    Vase(Vase),
    /// A squirrel, in the mini-game about finding them.
    Squirrel(SquirrelState),
    /// A Zen Garden tool being used, like the watering can.
    ZenTool(ZenTool),
    /// Stinky the Snail, in the Zen Garden.
    Stinky {
        state: StinkyState,
        /// Where he's crawling to.
        goal: Position,
    },
    Rake(RakeState),
    /// A brain at the end of a row in I, Zombie.
    IZombieBrain {
        /// Whether a zombie got to it (`Challenge.IZombieSquishBrain`).
        is_squished: bool,
    },
    AquariumShadow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortalShape {
    Circle,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vase {
    pub kind: VaseKind,
    pub content: Option<VaseContent>,
    /// Whether the mouse is over it.
    pub is_highlighted: bool,
    /// How see-through it is, which a nearby Plantern raises. It counts down, and the vase
    /// hides its contents again when it gets to 0.
    pub see_through: i32,
}

/// What a vase looks like, which hints at what's inside.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum VaseKind {
    /// A plain vase.
    Mystery = 3, // ScaryPotQuestion
    /// A green vase with a leaf, which holds a plant.
    Plant = 4, // ScaryPotLeaf
    /// A vase with a zombie's face.
    Zombie = 5, // ScaryPotZombie
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VaseContent {
    Plant(PlantType),
    Zombie(ZombieType),
    /// How much sun.
    Sun(i32),
}

/// What a squirrel is doing. Its values are the game's `GridItemState`'s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum SquirrelState {
    Waiting = 6,       // SquirrelWaiting
    Peeking = 7,       // SquirrelPeeking
    RunningUp = 8,     // SquirrelRunningUp
    RunningDown = 9,   // SquirrelRunningDown
    RunningLeft = 10,  // SquirrelRunningLeft
    RunningRight = 11, // SquirrelRunningRight
    Caught = 12,       // SquirrelCaught
    Zombie = 13,       // SquirrelZombie
}

/// Which Zen Garden tool is being used. Its values are the game's `GridItemState`'s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ZenTool {
    WateringCan = 14,     // ZenToolWateringCan
    Fertilizer = 15,      // ZenToolFertilizer
    BugSpray = 16,        // ZenToolBugSpray
    Phonograph = 17,      // ZenToolPhonograph
    GoldWateringCan = 18, // ZenToolGoldWateringCan
}

/// What Stinky the Snail is doing. Its values are the game's `GridItemState`'s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum StinkyState {
    WalkingLeft = 19,   // StinkyWalkingLeft
    TurningLeft = 20,   // StinkyTurningLeft
    WalkingRight = 21,  // StinkyWalkingRight
    TurningRight = 22,  // StinkyTurningRight
    Sleeping = 23,      // StinkySleeping
    FallingAsleep = 24, // StinkyFallingAsleep
    WakingUp = 25,      // StinkyWakingUp
}

/// What a rake is doing. Its values are the game's `GridItemState`'s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum RakeState {
    Attracting = 26, // RakeAttracting
    Waiting = 27,    // RakeWaiting
    Triggered = 28,  // RakeTriggered
}

/// The game's `GridItemState`. Each kind of item has its own states, which [`GridItemKind`]
/// holds; the ones that aren't there are only ever `Normal`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum GridItemState {
    Normal = 0,
    GravestoneSpecial = 1,
    PortalClosed = 2,
    ScaryPotQuestion = 3,
    ScaryPotLeaf = 4,
    ScaryPotZombie = 5,
    SquirrelWaiting = 6,
    SquirrelPeeking = 7,
    SquirrelRunningUp = 8,
    SquirrelRunningDown = 9,
    SquirrelRunningLeft = 10,
    SquirrelRunningRight = 11,
    SquirrelCaught = 12,
    SquirrelZombie = 13,
    ZenToolWateringCan = 14,
    ZenToolFertilizer = 15,
    ZenToolBugSpray = 16,
    ZenToolPhonograph = 17,
    ZenToolGoldWateringCan = 18,
    StinkyWalkingLeft = 19,
    StinkyTurningLeft = 20,
    StinkyWalkingRight = 21,
    StinkyTurningRight = 22,
    StinkySleeping = 23,
    StinkyFallingAsleep = 24,
    StinkyWakingUp = 25,
    RakeAttracting = 26,
    RakeWaiting = 27,
    RakeTriggered = 28,
    BrainSquished = 29,
}

impl Enum for GridItemState {}

impl GridItemState {
    /// The state as one kind of item's states, like a [`SquirrelState`], or `None` if it
    /// isn't one of them.
    fn of_kind<S: TryFrom<i32>>(self) -> Option<S> {
        S::try_from(self.into()).ok()
    }

    /// One kind of item's state, like a [`SquirrelState`], as a `GridItemState`.
    fn from_kind<S: Into<i32>>(state: S) -> Self {
        Self::try_from(state.into()).expect("every kind's states are `GridItemState`s")
    }
}

/// The game's `GridItemType`: a [`GridItemKind`] without its data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum GridItemType {
    Gravestone = 1,
    Crater = 2,
    Ladder = 3,
    PortalCircle = 4,
    PortalSquare = 5,
    Brain = 6,
    ScaryPot = 7,
    Squirrel = 8,
    ZenTool = 9,
    Stinky = 10,
    Rake = 11,
    IZombieBrain = 12,
    AquariumShadow = 13,
}

impl Enum for GridItemType {}

/// `GridItemType.None`.
impl EnumWithNone for GridItemType {
    const NONE: i32 = 0;
}

/// The game's `ScaryPotType`: what kind of thing a vase holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ScaryPotType {
    Seed = 1,
    Zombie = 2,
    Sun = 3,
}

impl Enum for ScaryPotType {}

/// `ScaryPotType.None`.
impl EnumWithNone for ScaryPotType {
    const NONE: i32 = 0;
}

class_layout! {
    /// `Reloaded.Gameplay.GridItem`.
    pub struct GridItemFields = "Reloaded.Gameplay.GridItem" {
        id: Id<GridItem> = "<DataID>k__BackingField",
        item_type: Option<GridItemType> = "mGridItemType",
        state: GridItemState = "mGridItemState",
        column: i32 = "mGridX",
        row: i32 = "mGridY",
        /// A gravestone's rise, or a crater's time left.
        counter: i32 = "mGridItemCounter",
        dead: bool = "mDead",
        x: f32 = "mPosX",
        y: f32 = "mPosY",
        goal_x: f32 = "mGoalX",
        goal_y: f32 = "mGoalY",
        // What a vase holds. A new grid item has `ZombieType.Invalid` and `SeedType.None`.
        zombie_type: Option<ZombieType> = "mZombieType",
        seed_type: Option<PlantType> = "mSeedType",
        scary_pot_type: Option<ScaryPotType> = "mScaryPotType",
        sun_count: i32 = "mSunCount",
        highlighted: bool = "mHighlighted",
        transparent_counter: i32 = "mTransparentCounter",
    }
}

impl GameObject for GridItem {
    type Fields = GridItemFields;

    /// `None` also while an item's state isn't yet one of its kind's, like a squirrel's.
    fn from_fields(fields: &GridItemFields) -> Option<Self> {
        let state = fields.state;
        let portal = |shape| GridItemKind::Portal {
            shape,
            is_closing: state == GridItemState::PortalClosed,
        };
        let kind = match fields.item_type? {
            GridItemType::Gravestone => GridItemKind::Gravestone {
                risen: fields.counter,
                is_special: state == GridItemState::GravestoneSpecial,
            },
            GridItemType::Crater => GridItemKind::Crater {
                disappears_in: Ticks(fields.counter),
            },
            GridItemType::Ladder => GridItemKind::Ladder,
            GridItemType::PortalCircle => portal(PortalShape::Circle),
            GridItemType::PortalSquare => portal(PortalShape::Square),
            GridItemType::Brain => GridItemKind::Brain,
            GridItemType::ScaryPot => GridItemKind::Vase(Vase {
                kind: state.of_kind()?,
                content: match fields.scary_pot_type {
                    Some(ScaryPotType::Seed) => Some(VaseContent::Plant(fields.seed_type?)),
                    Some(ScaryPotType::Zombie) => Some(VaseContent::Zombie(fields.zombie_type?)),
                    Some(ScaryPotType::Sun) => Some(VaseContent::Sun(fields.sun_count)),
                    None => None,
                },
                is_highlighted: fields.highlighted,
                see_through: fields.transparent_counter,
            }),
            GridItemType::Squirrel => GridItemKind::Squirrel(state.of_kind()?),
            GridItemType::ZenTool => GridItemKind::ZenTool(state.of_kind()?),
            GridItemType::Stinky => GridItemKind::Stinky {
                state: state.of_kind()?,
                goal: Position {
                    x: fields.goal_x,
                    y: fields.goal_y,
                },
            },
            GridItemType::Rake => GridItemKind::Rake(state.of_kind()?),
            GridItemType::IZombieBrain => GridItemKind::IZombieBrain {
                is_squished: state == GridItemState::BrainSquished,
            },
            GridItemType::AquariumShadow => GridItemKind::AquariumShadow,
        };
        Some(Self {
            kind,
            cell: Cell {
                row: fields.row,
                column: fields.column,
            },
            position: Position {
                x: fields.x,
                y: fields.y,
            },
        })
    }

    fn update_fields(&self, fields: &mut GridItemFields) {
        let normal_or = |state, is_it| {
            if is_it { state } else { GridItemState::Normal }
        };
        let (item_type, state) = match self.kind {
            GridItemKind::Gravestone { risen, is_special } => {
                fields.counter = risen;
                (
                    GridItemType::Gravestone,
                    normal_or(GridItemState::GravestoneSpecial, is_special),
                )
            }
            GridItemKind::Crater { disappears_in } => {
                fields.counter = disappears_in.0;
                (GridItemType::Crater, GridItemState::Normal)
            }
            GridItemKind::Ladder => (GridItemType::Ladder, GridItemState::Normal),
            GridItemKind::Portal { shape, is_closing } => (
                match shape {
                    PortalShape::Circle => GridItemType::PortalCircle,
                    PortalShape::Square => GridItemType::PortalSquare,
                },
                normal_or(GridItemState::PortalClosed, is_closing),
            ),
            GridItemKind::Brain => (GridItemType::Brain, GridItemState::Normal),
            GridItemKind::Vase(vase) => {
                fields.scary_pot_type = vase.content.map(|content| match content {
                    VaseContent::Plant(plant) => {
                        fields.seed_type = Some(plant);
                        ScaryPotType::Seed
                    }
                    VaseContent::Zombie(zombie) => {
                        fields.zombie_type = Some(zombie);
                        ScaryPotType::Zombie
                    }
                    VaseContent::Sun(sun) => {
                        fields.sun_count = sun;
                        ScaryPotType::Sun
                    }
                });
                fields.highlighted = vase.is_highlighted;
                fields.transparent_counter = vase.see_through;
                (GridItemType::ScaryPot, GridItemState::from_kind(vase.kind))
            }
            GridItemKind::Squirrel(squirrel) => {
                (GridItemType::Squirrel, GridItemState::from_kind(squirrel))
            }
            GridItemKind::ZenTool(tool) => (GridItemType::ZenTool, GridItemState::from_kind(tool)),
            GridItemKind::Stinky { state, goal } => {
                fields.goal_x = goal.x;
                fields.goal_y = goal.y;
                (GridItemType::Stinky, GridItemState::from_kind(state))
            }
            GridItemKind::Rake(rake) => (GridItemType::Rake, GridItemState::from_kind(rake)),
            GridItemKind::IZombieBrain { is_squished } => (
                GridItemType::IZombieBrain,
                normal_or(GridItemState::BrainSquished, is_squished),
            ),
            GridItemKind::AquariumShadow => (GridItemType::AquariumShadow, GridItemState::Normal),
        };
        fields.item_type = Some(item_type);
        fields.state = state;
        fields.row = self.cell.row;
        fields.column = self.cell.column;
        fields.x = self.position.x;
        fields.y = self.position.y;
    }
}

impl BoardObject for GridItem {
    fn id(fields: &GridItemFields) -> Id<GridItem> {
        fields.id
    }

    fn is_dead(fields: &GridItemFields) -> bool {
        fields.dead
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::entity::assert_round_trips;
    use crate::entities::id::test_id;

    fn fields(item_type: GridItemType, state: GridItemState) -> GridItemFields {
        GridItemFields {
            id: test_id(1),
            item_type: Some(item_type),
            state,
            column: 0,
            row: 0,
            counter: 0,
            dead: false,
            x: 0.0,
            y: 0.0,
            goal_x: 0.0,
            goal_y: 0.0,
            zombie_type: None,
            seed_type: None,
            scary_pot_type: None,
            sun_count: 0,
            highlighted: false,
            transparent_counter: 0,
        }
    }

    /// Fields that differ from [`fields`]'s in everything a grid item has.
    fn elsewhere(item_type: GridItemType, state: GridItemState) -> GridItemFields {
        GridItemFields {
            column: 4,
            row: 2,
            x: 1.0,
            y: 2.0,
            ..fields(item_type, state)
        }
    }

    #[test]
    fn every_kinds_states_are_grid_item_states() {
        fn check<S: TryFrom<i32> + Into<i32> + Copy>() {
            for value in 0..=40 {
                if let Ok(state) = S::try_from(value) {
                    assert_eq!(i32::from(GridItemState::from_kind(state)), value);
                }
            }
        }
        check::<VaseKind>();
        check::<SquirrelState>();
        check::<ZenTool>();
        check::<StinkyState>();
        check::<RakeState>();
    }

    #[test]
    fn vases_round_trip() {
        let plant_vase = GridItemFields {
            scary_pot_type: Some(ScaryPotType::Seed),
            seed_type: Some(PlantType::Squash),
            ..fields(GridItemType::ScaryPot, GridItemState::ScaryPotLeaf)
        };
        let sun_vase = GridItemFields {
            scary_pot_type: Some(ScaryPotType::Sun),
            sun_count: 75,
            highlighted: true,
            transparent_counter: 50,
            ..elsewhere(GridItemType::ScaryPot, GridItemState::ScaryPotQuestion)
        };
        assert_round_trips::<GridItem>(plant_vase.clone(), sun_vase);

        let zombie_vase = GridItemFields {
            scary_pot_type: Some(ScaryPotType::Zombie),
            zombie_type: Some(ZombieType::Gargantuar),
            highlighted: true,
            transparent_counter: 50,
            ..elsewhere(GridItemType::ScaryPot, GridItemState::ScaryPotZombie)
        };
        assert_round_trips::<GridItem>(plant_vase, zombie_vase.clone());

        let vase_being_filled = GridItemFields {
            zombie_type: None,
            ..zombie_vase
        };
        assert_eq!(GridItem::from_fields(&vase_being_filled), None);
    }

    #[test]
    fn items_with_counters_round_trip() {
        let gravestone = GridItemFields {
            counter: 40,
            ..fields(GridItemType::Gravestone, GridItemState::Normal)
        };
        let special_gravestone = GridItemFields {
            counter: 100,
            ..elsewhere(GridItemType::Gravestone, GridItemState::GravestoneSpecial)
        };
        assert_round_trips::<GridItem>(gravestone.clone(), special_gravestone);
        let crater = GridItemFields {
            counter: 3000,
            ..elsewhere(GridItemType::Crater, GridItemState::Normal)
        };
        assert_round_trips::<GridItem>(gravestone, crater);
    }

    #[test]
    fn portals_round_trip() {
        let open_circle = fields(GridItemType::PortalCircle, GridItemState::Normal);
        let closing_square = elsewhere(GridItemType::PortalSquare, GridItemState::PortalClosed);
        assert_round_trips::<GridItem>(open_circle, closing_square);
    }

    #[test]
    fn items_with_their_own_states_round_trip() {
        let rake = fields(GridItemType::Rake, GridItemState::RakeWaiting);
        let stinky = GridItemFields {
            goal_x: 5.0,
            goal_y: 6.0,
            ..elsewhere(GridItemType::Stinky, GridItemState::StinkySleeping)
        };
        assert_round_trips::<GridItem>(rake.clone(), stinky);
        let squirrel = elsewhere(GridItemType::Squirrel, GridItemState::SquirrelPeeking);
        assert_round_trips::<GridItem>(rake.clone(), squirrel);
        let tool = elsewhere(GridItemType::ZenTool, GridItemState::ZenToolBugSpray);
        assert_round_trips::<GridItem>(rake.clone(), tool);
        let squished_brain = elsewhere(GridItemType::IZombieBrain, GridItemState::BrainSquished);
        assert_round_trips::<GridItem>(rake, squished_brain);
    }

    #[test]
    fn items_set_up_without_their_kinds_state_are_unfinished() {
        let new_squirrel = fields(GridItemType::Squirrel, GridItemState::Normal);
        assert_eq!(GridItem::from_fields(&new_squirrel), None);
    }
}
