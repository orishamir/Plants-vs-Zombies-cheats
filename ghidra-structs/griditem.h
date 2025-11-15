#include "plant.h"
#include "zombie.h"
#include <stdint.h>

typedef enum GriditemType
{
    GraveBuster = 1,
    DoomShroomCrater = 2,
    Portal = 5,
    /// The ones you drop the zombies in "Zombiquarium"
    EatableBrain = 6,
    Vase = 7,
    // WateringCan / BugSpray / MusicPlayer / Chocolate
    ZenGardenItem = 9,
    Snail = 10,
    Rake = 11,
    // The brain in the reverse-zombie puzzle thingy
    Brain = 12,
} GriditemType;

typedef enum VaseKind
{
    Mistery = 3,
    Plant = 4,
    Zombie = 5,
} VaseKind;

typedef enum VaseContent
{
    Plant = 1,
    Zombie = 2,
    Sun = 3,
} VaseContent;

typedef struct Griditem
{
    char pad1[0x68 + 4 + 4];
    GriditemType griditem_type;
    VaseKind vase_kind;
    uint32_t column;
    uint32_t row;
    char pad2[8];
    float posX;
    float posY;
    float destinationX;
    float destionationY;
    char pad3[8];
    ZombieType zombie_type;
    PlantType plant_type;
    VaseContent vase_content;
    bool is_highlighted;
    char pad4[3];
    uint32_t opacity;
    uint32_t sun_count;
    char pad5[8 * 5];
} Griditem;
