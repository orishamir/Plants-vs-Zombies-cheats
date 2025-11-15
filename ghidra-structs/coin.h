#include <stdint.h>

typedef enum CoinType
{
    Silver = 1,
    Gold = 2,
    Diamond = 3,
    Sun = 4,
    MiniSun = 5,
    BigSun = 6,
    DroppedCard = 16,
    GiantBagOfCash = 18,
    /// Beating a mini-game, Puzzle Mode, or Survival Mode level for the first time.
    // Trophy = ?
    /// Beating Adventure Mode for the first time.
    // SilverSunflowerTrophy = ?
    /// Beating every mini-game, puzzle, and Survival level for the first time
    GoldSunflowerTrophy = 22,
    /// You can pick up chocolates
    Chocolate = 23,
} CoinType;

typedef struct Coin
{
    char pad1[0xb0 - 1];
    CoinType coin_type;
} Coin;