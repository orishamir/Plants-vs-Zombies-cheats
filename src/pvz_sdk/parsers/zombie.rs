use crate::ReaderAt;
use crate::models::Zombie;
use crate::offsets::ZombieOffset;
use crate::traits::{ReadEntityError, ReadableEntity};

impl ReadableEntity for Zombie {
    const SIZE: usize = 360;

    fn read(reader: ReaderAt) -> Result<Self, ReadEntityError> {
        assert_eq!(reader.len(), Self::SIZE);

        Ok(Self {
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
