#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct Entities {
    // Zombies
    pub zombies_ptr: u32,
    _pad1: [u8; 8],
    pub next_zombie_idx: u32,
    pub zombies_count: u32,
    _pad2: [u8; 8],

    // Plants
    plants_ptr: u32,
    _pad3: [u8; 8],
    next_plant_idx: u32,
    plants_count: u32,
    _pad4: [u8; 8],

    // Projectiles
    projectiles_ptr: u32,
    _pad5: [u8; 8],
    next_projectile_idx: u32,
    projectiles_count: u32,
    _pad6: [u8; 8],

    // Coins
    coins_ptr: u32,
    _pad7: [u8; 8],
    next_coin_idx: u32,
    coins_count: u32,
    _pad8: [u8; 8],

    // Lawnmowers
    lawnmower_ptr: u32,
    _pad9: [u8; 8],
    next_lawnmower_idx: u32,
    lawnmower_count: u32,
    _pad10: [u8; 8],

    // GridItems
    griditems_ptr: u32,
    _pad11: [u8; 8],
    next_griditems_idx: u32,
    griditems_count: u32,
    _pad12: [u8; 8],
}
