use proc_mem::ProcMemError;

use crate::game::Popcapgame;
use crate::models::{Card, Coin, Entities, Griditem, Lawnmower, Plant, Projectile, Zombie};
use crate::traits::ReadableEntity;

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

impl EntitiesLoader {
    pub fn load(game: &Popcapgame) -> Result<Self, ProcMemError> {
        let ents = game.read_with_base_addr::<Entities>(&[
            0x32f39c, 0x540, 0x48c, 0x0, 0x3dc, 0x4, 0x0, 0xa4,
        ])?;

        let zombies =
            Self::load_entity::<Zombie>(game, ents.zombies_ptr, ents.zombies_count, |zombie| {
                !zombie.is_dead
            });

        let plants =
            Self::load_entity::<Plant>(game, ents.plants_ptr, ents.plants_count, |plant| {
                !plant.is_deleted
            });

        let projectiles = Self::load_entity::<Projectile>(
            game,
            ents.projectiles_ptr,
            ents.projectiles_count,
            |projectile| !projectile.is_deleted,
        );

        let coins = Self::load_entity::<Coin>(game, ents.coins_ptr, ents.coins_count, |coin| {
            !coin.is_deleted
        });

        let lawnmowers = Self::load_entity::<Lawnmower>(
            game,
            ents.lawnmower_ptr,
            ents.lawnmower_count,
            |lawnmower| !lawnmower.is_deleted,
        );

        let griditems = Self::load_entity::<Griditem>(
            game,
            ents.griditems_ptr,
            ents.griditems_count,
            |griditem| !griditem.is_deleted,
        );

        let cards = Self::load_cards(game)?;

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

    pub fn load_entity<T: ReadableEntity>(
        game: &Popcapgame,
        base_ptr: usize,
        ent_count: u32,
        filter: impl Fn(&T) -> bool,
    ) -> Vec<T> {
        let mut tmp_ptr = base_ptr;
        std::iter::from_fn(move || {
            let ent = game.read_parseable_at::<T>(tmp_ptr).unwrap();
            tmp_ptr += T::size_of();
            Some(ent)
        })
        .filter(filter)
        .take(ent_count as usize)
        .collect()
    }

    fn load_cards(game: &Popcapgame) -> Result<Vec<Card>, ProcMemError> {
        let cards_count: usize =
            game.read_with_base_addr(&[0x331C50, 0x320, 0x18, 0x0, 0x8, 0x15c, 0x24])?;

        let cards: Vec<Card> = (0..cards_count)
            .map(|i| {
                game.read_parseable_with_base_addr::<Card>(&[
                    0x331C50,
                    0x320,
                    0x18,
                    0x0,
                    0x8,
                    0x15c,
                    0x28 + i * Card::size_of(),
                ])
                .unwrap()
            })
            .collect();

        assert_eq!(cards_count, cards.len(), "Couldn't get all cards");

        Ok(cards)
    }
}
