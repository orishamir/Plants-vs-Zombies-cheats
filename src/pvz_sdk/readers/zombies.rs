use crate::Popcapgame;
use crate::entities::{Zombie, Zombies};
use crate::offsets::{EntityInformationOffset, EntityOffset, ZombieOffset};
use crate::readers::OffsetReader;
use crate::readers::traits::{ReadEntityError, ReadableEntity};

const ZOMBIE_MEMORY_SIZE: usize = 360;

impl ReadableEntity for Zombies {
    fn read(game: &Popcapgame) -> Result<Self, ReadEntityError> {
        let base_addr = game.read_ptr_chain(
            &[
                0x32f39c,
                0x540,
                0x48c,
                0x0,
                0x3dc,
                0x4,
                0x0,
                EntityOffset::Zombies.into(),
            ],
            true,
        )?;
        let reader = OffsetReader::new(base_addr, game);

        let capacity = reader.read_u32(EntityInformationOffset::Capacity)?;
        let array_addr = reader.read_usize(EntityInformationOffset::ArrayPtr)?;

        let zombies: Vec<Zombie> = (0..capacity)
            .filter_map(|i| {
                let entity_reader =
                    OffsetReader::new(array_addr + i as usize * ZOMBIE_MEMORY_SIZE, game);
                Self::read_zombie(&entity_reader).ok()
            })
            .filter(|zombie| !zombie.is_dead)
            .collect();

        Ok(Self {
            capacity,
            next_index: reader.read_u32(EntityInformationOffset::NextIndex)?,
            count: reader.read_u32(EntityInformationOffset::Count)?,
            zombies,
        })
    }
}

impl Zombies {
    fn read_zombie(reader: &OffsetReader) -> Result<Zombie, ReadEntityError> {
        Ok(Zombie {
            display_pos_x: reader.read_u32(ZombieOffset::DisplayPosX)?,
            display_pos_y: reader.read_u32(ZombieOffset::DisplayPosY)?,
            row: reader.read_u32(ZombieOffset::Row)?,
            zombie_type: reader.read_u32(ZombieOffset::ZombieType)?.try_into()?,
            pos_x: reader.read_f32(ZombieOffset::PosX)?,
            pos_y: reader.read_f32(ZombieOffset::PosY)?,
            freeze_timer: reader.read_u32(ZombieOffset::FreezeTimer)?,
            is_hypnotized: reader.read_bool(ZombieOffset::IsHypnotized)?,
            armor_type: reader.read_u32(ZombieOffset::ArmorType)?.try_into()?,
            health: reader.read_i32(ZombieOffset::Health)?,
            original_health: reader.read_i32(ZombieOffset::OriginalHealth)?,
            armor_hp: reader.read_u32(ZombieOffset::ArmorHealth)?,
            original_armor_hp: reader.read_u32(ZombieOffset::OriginalArmorHealth)?,
            is_dead: reader.read_bool(ZombieOffset::IsDead)?,
        })
    }
}
