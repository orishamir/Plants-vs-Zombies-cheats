use proc_mem::ProcMemError;

use crate::pvz_sdk::cheated_entity::CheatedEntity;
use crate::pvz_sdk::game::Popcapgame;
use crate::pvz_sdk::models::{
    Card, Coin, Entities, Griditem, Lawnmower, Plant, Projectile, Zombie,
};
use crate::pvz_sdk::traits::{ReadableEntity, WriteableEntity};

#[allow(dead_code)]
pub struct EntitiesLoader {
    pub zombies: Vec<CheatedEntity<Zombie>>,
    pub plants: Vec<CheatedEntity<Plant>>,
    pub projectiles: Vec<CheatedEntity<Projectile>>,
    pub coins: Vec<CheatedEntity<Coin>>,
    pub lawnmowers: Vec<CheatedEntity<Lawnmower>>,
    pub griditems: Vec<CheatedEntity<Griditem>>,
    pub cards: Vec<CheatedEntity<Card>>,
}

impl EntitiesLoader {
    pub fn load(game: &Popcapgame) -> Result<Self, ProcMemError> {
        let ents = game
            .read::<Entities>(&[0x32f39c, 0x540, 0x48c, 0x0, 0x3dc, 0x4, 0x0, 0xa4], true)
            .unwrap();

        let zombies = Self::load_cheated_entity::<Zombie>(
            game,
            ents.zombies_ptr,
            ents.zombies_count,
            |zombie| !zombie.is_dead,
        );

        let plants =
            Self::load_cheated_entity::<Plant>(game, ents.plants_ptr, ents.plants_count, |plant| {
                !plant.is_deleted
            });

        let projectiles = Self::load_cheated_entity::<Projectile>(
            game,
            ents.projectiles_ptr,
            ents.projectiles_count,
            |projectile| !projectile.is_deleted,
        );

        let coins =
            Self::load_cheated_entity::<Coin>(game, ents.coins_ptr, ents.coins_count, |coin| {
                !coin.is_deleted
            });

        let lawnmowers = Self::load_cheated_entity::<Lawnmower>(
            game,
            ents.lawnmower_ptr,
            ents.lawnmower_count,
            |lawnmower| !lawnmower.is_deleted,
        );

        let griditems = Self::load_cheated_entity::<Griditem>(
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

    #[allow(dead_code)]
    pub fn load_entity<T: ReadableEntity>(
        game: &Popcapgame,
        base_ptr: usize,
        ent_count: u32,
        filter: impl Fn(&T) -> bool,
    ) -> Vec<T> {
        let mut tmp_ptr = base_ptr;
        std::iter::from_fn(move || {
            let ent = game.read_entity_at::<T>(tmp_ptr).unwrap();
            tmp_ptr += T::SIZE;
            Some(ent)
        })
        .filter(filter)
        .take(ent_count as usize)
        .collect()
    }

    pub fn load_cheated_entity<T: ReadableEntity + WriteableEntity>(
        game: &Popcapgame,
        base_ptr: usize,
        ent_count: u32,
        filter: impl Fn(&T) -> bool,
    ) -> Vec<CheatedEntity<T>> {
        let mut tmp_ptr = base_ptr;
        std::iter::from_fn(move || {
            let ent = game.read_entity_at::<T>(tmp_ptr).unwrap();
            let ret = Some(CheatedEntity::new(tmp_ptr, ent));

            tmp_ptr += T::SIZE;

            ret
        })
        .filter(|cheated_ent| filter(&cheated_ent.entity))
        .take(ent_count as usize)
        .collect()
    }

    fn load_cards(game: &Popcapgame) -> Result<Vec<CheatedEntity<Card>>, ProcMemError> {
        let cards_count: usize =
            game.read(&[0x331C50, 0x320, 0x18, 0x0, 0x8, 0x15c, 0x24], true)?;

        let cards: Vec<CheatedEntity<Card>> = (0..cards_count)
            .map(|i| {
                let addr = game
                    .read_ptr_chain(
                        &[
                            0x331C50,
                            0x320,
                            0x18,
                            0x0,
                            0x8,
                            0x15c,
                            0x28 + i * Card::SIZE,
                        ],
                        true,
                    )
                    .unwrap();
                let card = game.read_entity_at::<Card>(addr).unwrap();

                CheatedEntity::new(addr, card)
            })
            .collect();

        assert_eq!(cards_count, cards.len(), "Couldn't get all cards");

        Ok(cards)
    }
}
