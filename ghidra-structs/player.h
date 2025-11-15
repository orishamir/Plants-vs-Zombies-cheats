#include <stdint.h>

typedef struct Player
{
    char pad1[0x10 - 1];
    uint32_t sun_count;
} Player;