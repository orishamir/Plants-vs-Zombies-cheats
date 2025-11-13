#include "griditem.h"
#include <stdint.h>

typedef struct World
{
    char pad1[0x170 - 1];
    Griditem *griditem;
} World;
