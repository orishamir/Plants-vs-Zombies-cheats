#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Entities {
    // Zombies
    pub zombies_ptr: usize,
    _pad1: [u8; 8],
    pub next_zombie_idx: u32,
    pub zombies_count: u32,
    _pad2: [u8; 8],

    // Plants
    pub plants_ptr: usize,
    _pad3: [u8; 8],
    pub next_plant_idx: u32,
    pub plants_count: u32,
    _pad4: [u8; 8],

    // Projectiles
    pub projectiles_ptr: usize,
    _pad5: [u8; 8],
    pub next_projectile_idx: u32,
    pub projectiles_count: u32,
    _pad6: [u8; 8],

    // Coins
    pub coins_ptr: usize,
    _pad7: [u8; 8],
    pub next_coin_idx: u32,
    pub coins_count: u32,
    _pad8: [u8; 8],

    // Lawnmowers
    pub lawnmower_ptr: usize,
    _pad9: [u8; 8],
    pub next_lawnmower_idx: u32,
    pub lawnmower_count: u32,
    _pad10: [u8; 8],

    // GridItems
    pub griditems_ptr: usize,
    _pad11: [u8; 8],
    pub next_griditems_idx: u32,
    pub griditems_count: u32,
    _pad12: [u8; 8],
}
