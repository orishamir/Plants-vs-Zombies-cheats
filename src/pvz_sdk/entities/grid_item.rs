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
    /// What it's doing. The states are each for one kind of item, like a vase's look.
    pub state: GridItemState,
}

/// What a [`GridItem`] is, with what's only stored for that kind of item.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridItemKind {
    Gravestone {
        /// How far it has risen out of the ground, up to 100.
        risen: i32,
    },
    /// The hole a Doom-shroom leaves, which nothing can be planted in until it's gone.
    Crater {
        disappears_in: Ticks,
    },
    /// A ladder that a Ladder Zombie put on a plant.
    Ladder,
    /// A portal in Portal Combat.
    PortalCircle,
    PortalSquare,
    /// A brain dropped in Zombiquarium.
    Brain,
    /// A vase in Vasebreaker. The game calls it a scary pot.
    Vase(Vase),
    Squirrel,
    /// A Zen Garden tool being used, like the watering can.
    ZenTool,
    /// Stinky the Snail, in the Zen Garden.
    Stinky {
        /// Where he's crawling to.
        goal: Position,
    },
    Rake,
    /// A brain at the end of a row in I, Zombie.
    IZombieBrain,
    AquariumShadow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vase {
    pub content: Option<VaseContent>,
    /// Whether the mouse is over it.
    pub is_highlighted: bool,
    /// How see-through it is, which a nearby Plantern raises. It counts down, and the vase
    /// hides its contents again when it gets to 0.
    pub see_through: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VaseContent {
    Plant(PlantType),
    Zombie(ZombieType),
    /// How much sun.
    Sun(i32),
}

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

    fn from_fields(fields: &GridItemFields) -> Option<Self> {
        let kind = match fields.item_type? {
            GridItemType::Gravestone => GridItemKind::Gravestone {
                risen: fields.counter,
            },
            GridItemType::Crater => GridItemKind::Crater {
                disappears_in: Ticks(fields.counter),
            },
            GridItemType::Ladder => GridItemKind::Ladder,
            GridItemType::PortalCircle => GridItemKind::PortalCircle,
            GridItemType::PortalSquare => GridItemKind::PortalSquare,
            GridItemType::Brain => GridItemKind::Brain,
            GridItemType::ScaryPot => GridItemKind::Vase(Vase {
                content: match fields.scary_pot_type {
                    Some(ScaryPotType::Seed) => Some(VaseContent::Plant(fields.seed_type?)),
                    Some(ScaryPotType::Zombie) => Some(VaseContent::Zombie(fields.zombie_type?)),
                    Some(ScaryPotType::Sun) => Some(VaseContent::Sun(fields.sun_count)),
                    None => None,
                },
                is_highlighted: fields.highlighted,
                see_through: fields.transparent_counter,
            }),
            GridItemType::Squirrel => GridItemKind::Squirrel,
            GridItemType::ZenTool => GridItemKind::ZenTool,
            GridItemType::Stinky => GridItemKind::Stinky {
                goal: Position {
                    x: fields.goal_x,
                    y: fields.goal_y,
                },
            },
            GridItemType::Rake => GridItemKind::Rake,
            GridItemType::IZombieBrain => GridItemKind::IZombieBrain,
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
            state: fields.state,
        })
    }

    fn update_fields(&self, fields: &mut GridItemFields) {
        fields.item_type = Some(match self.kind {
            GridItemKind::Gravestone { risen } => {
                fields.counter = risen;
                GridItemType::Gravestone
            }
            GridItemKind::Crater { disappears_in } => {
                fields.counter = disappears_in.0;
                GridItemType::Crater
            }
            GridItemKind::Ladder => GridItemType::Ladder,
            GridItemKind::PortalCircle => GridItemType::PortalCircle,
            GridItemKind::PortalSquare => GridItemType::PortalSquare,
            GridItemKind::Brain => GridItemType::Brain,
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
                GridItemType::ScaryPot
            }
            GridItemKind::Squirrel => GridItemType::Squirrel,
            GridItemKind::ZenTool => GridItemType::ZenTool,
            GridItemKind::Stinky { goal } => {
                fields.goal_x = goal.x;
                fields.goal_y = goal.y;
                GridItemType::Stinky
            }
            GridItemKind::Rake => GridItemType::Rake,
            GridItemKind::IZombieBrain => GridItemType::IZombieBrain,
            GridItemKind::AquariumShadow => GridItemType::AquariumShadow,
        });
        fields.row = self.cell.row;
        fields.column = self.cell.column;
        fields.x = self.position.x;
        fields.y = self.position.y;
        fields.state = self.state;
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

    #[test]
    fn vases_round_trip() {
        let plant_vase = GridItemFields {
            scary_pot_type: Some(ScaryPotType::Seed),
            seed_type: Some(PlantType::Squash),
            ..fields(GridItemType::ScaryPot, GridItemState::ScaryPotLeaf)
        };
        let sun_vase = GridItemFields {
            column: 4,
            row: 2,
            x: 1.0,
            y: 2.0,
            scary_pot_type: Some(ScaryPotType::Sun),
            sun_count: 75,
            highlighted: true,
            transparent_counter: 50,
            ..fields(GridItemType::ScaryPot, GridItemState::ScaryPotQuestion)
        };
        assert_round_trips::<GridItem>(plant_vase.clone(), sun_vase);

        let zombie_vase = GridItemFields {
            column: 5,
            row: 3,
            x: 3.0,
            y: 4.0,
            scary_pot_type: Some(ScaryPotType::Zombie),
            zombie_type: Some(ZombieType::Gargantuar),
            highlighted: true,
            transparent_counter: 50,
            ..fields(GridItemType::ScaryPot, GridItemState::ScaryPotZombie)
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
        let crater = GridItemFields {
            counter: 3000,
            column: 1,
            row: 1,
            x: 1.0,
            y: 1.0,
            ..fields(GridItemType::Crater, GridItemState::GravestoneSpecial)
        };
        assert_round_trips::<GridItem>(gravestone, crater);
    }
}
