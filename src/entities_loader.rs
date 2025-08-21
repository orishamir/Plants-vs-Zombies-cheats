use crate::game::GameProcess;
use crate::models::{Card, Coin, Entities, Griditem, Plant, Projectile, Zombie};

#[allow(dead_code)]
pub struct EntitiesLoader {
    pub zombies: Vec<Zombie>,
    pub plants: Vec<Plant>,
    pub projectiles: Vec<Projectile>,
    pub coins: Vec<Coin>,
    // lawnmowers
    pub griditems: Vec<Griditem>,
    pub cards: Vec<Card>,
}

impl EntitiesLoader {
    pub fn load(proc: &GameProcess) -> Result<Self, std::io::Error> {
        let ents = proc
            .read_with_base_addr::<Entities>(&[0x32f39c, 0x540, 0x48c, 0x0, 0x3dc, 0x4, 0x0, 0xa4])
            .expect("what");

        let zombies = Self::load_entity::<Zombie>(
            proc,
            ents.zombies_ptr,
            ents.zombies_count as usize,
            |zombie| !zombie.is_dead,
        );

        let plants = Self::load_entity::<Plant>(
            proc,
            ents.plants_ptr,
            ents.plants_count as usize,
            |plant| !plant.is_deleted,
        );

        let projectiles = Self::load_entity::<Projectile>(
            proc,
            ents.projectiles_ptr,
            ents.projectiles_count as usize,
            |projectile| !projectile.is_deleted,
        );

        let coins =
            Self::load_entity::<Coin>(proc, ents.coins_ptr, ents.coins_count as usize, |coin| {
                !coin.is_deleted
            });

        let griditems = Self::load_entity::<Griditem>(
            proc,
            ents.griditems_ptr,
            ents.griditems_count as usize,
            |griditem| !griditem.is_deleted,
        );

        let cards_count: usize = proc
            .read_with_base_addr(&[0x331C50, 0x320, 0x30, 0x0, 0x8, 0x15c, 0x24])
            .expect("msg");

        let cards = (0..cards_count)
            .map(|i| {
                proc.read_with_base_addr::<Card>(&[
                    0x331C50,
                    0x320,
                    0x30,
                    0x0,
                    0x8,
                    0x15c,
                    0x28 + i * size_of::<Card>(),
                ])
                .expect("Couldn't read card")
            })
            .collect();

        Ok(Self {
            zombies,
            plants,
            projectiles,
            coins,
            cards,
            griditems,
        })
    }

    fn load_entity<T: Copy>(
        proc: &GameProcess,
        base_ptr: usize,
        ent_count: usize,
        filter: impl Fn(&T) -> bool,
    ) -> Vec<T> {
        let mut tmp_ptr = base_ptr;
        std::iter::from_fn(move || {
            let zombie_or_err = proc.read_at::<T>(tmp_ptr);
            tmp_ptr += size_of::<T>();

            // TODO: Error handling?
            zombie_or_err.ok()
        })
        .filter(filter)
        .take(ent_count)
        .collect()
    }
}
