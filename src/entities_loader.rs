use thiserror::Error;

use crate::game::GameProcess;
use crate::models::{Card, Coin, Entities, Griditem, Lawnmower, Plant, Projectile, Zombie};

#[allow(dead_code)]
pub struct EntitiesLoader {
    pub zombies: Vec<Zombie>,
    pub plants: Vec<Plant>,
    pub projectiles: Vec<Projectile>,
    pub coins: Vec<Coin>,
    pub lawnmowers: Vec<Lawnmower>,
    pub griditems: Vec<Griditem>,
    pub cards: Vec<Card>,
}

#[derive(Error, Debug)]
pub enum LoadEntityError {
    #[error("not inside an active game")]
    NotInGame,
    #[error("io error: {0}")]
    Unknown(#[from] std::io::Error),
}

impl EntitiesLoader {
    pub fn load(proc: &GameProcess) -> Result<Self, LoadEntityError> {
        let ents = proc
            .read_with_base_addr::<Entities>(&[0x32f39c, 0x540, 0x48c, 0x0, 0x3dc, 0x4, 0x0, 0xa4])
            .map_err(|_| LoadEntityError::NotInGame)?;

        let zombies =
            Self::load_entity::<Zombie>(proc, ents.zombies_ptr, ents.zombies_count, |zombie| {
                !zombie.is_dead
            });

        let plants =
            Self::load_entity::<Plant>(proc, ents.plants_ptr, ents.plants_count, |plant| {
                !plant.is_deleted
            });

        let projectiles = Self::load_entity::<Projectile>(
            proc,
            ents.projectiles_ptr,
            ents.projectiles_count,
            |projectile| !projectile.is_deleted,
        );

        let coins = Self::load_entity::<Coin>(proc, ents.coins_ptr, ents.coins_count, |coin| {
            !coin.is_deleted
        });

        let lawnmowers = Self::load_entity::<Lawnmower>(
            proc,
            ents.lawnmower_ptr,
            ents.lawnmower_count,
            |lawnmower| !lawnmower.is_deleted,
        );

        let griditems = Self::load_entity::<Griditem>(
            proc,
            ents.griditems_ptr,
            ents.griditems_count,
            |griditem| !griditem.is_deleted,
        );

        let cards = Self::load_cards(proc)?;

        Ok(Self {
            zombies,
            plants,
            projectiles,
            coins,
            lawnmowers,
            griditems,
            cards,
        })
    }

    fn load_entity<T: Copy>(
        proc: &GameProcess,
        base_ptr: usize,
        ent_count: u32,
        filter: impl Fn(&T) -> bool,
    ) -> Vec<T> {
        let mut tmp_ptr = base_ptr;
        std::iter::from_fn(move || {
            let entity_or_err = proc.read_at::<T>(tmp_ptr);
            tmp_ptr += size_of::<T>();
            // TODO: Error handling?
            if let Ok(entity) = entity_or_err {
                Some(entity)
            } else {
                eprintln!(
                    "WARNING: Couldn't load entity at {:x}\nsize_of:{}\n because: {}",
                    tmp_ptr - size_of::<T>(),
                    size_of::<T>(),
                    entity_or_err.err().unwrap()
                );
                None
            }
        })
        .filter(filter)
        .take(ent_count as usize)
        .collect()
    }

    fn load_cards(proc: &GameProcess) -> Result<Vec<Card>, std::io::Error> {
        let cards_count: usize =
            proc.read_with_base_addr(&[0x331C50, 0x320, 0x18, 0x0, 0x8, 0x15c, 0x24])?;

        let cards: Vec<Card> = (0..cards_count)
            .filter_map(|i| {
                proc.read_with_base_addr::<Card>(&[
                    0x331C50,
                    0x320,
                    0x18,
                    0x0,
                    0x8,
                    0x15c,
                    0x28 + i * size_of::<Card>(),
                ])
                .ok()
            })
            .collect();

        assert_eq!(cards_count, cards.len(), "Couldn't get all cards");

        Ok(cards)
    }
}
