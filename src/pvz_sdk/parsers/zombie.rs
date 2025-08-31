use crate::ReaderAt;
use crate::models::Zombie;
use crate::offsets::ZombieOffset;
use crate::traits::ReadableEntity;

impl ReadableEntity for Zombie {
    const SIZE: usize = 360;

    fn read(reader: ReaderAt) -> Self {
        assert_eq!(reader.len(), Self::SIZE);

        Self {
            display_pos_x: reader.read_u32(ZombieOffset::DisplayPosX).unwrap(),
            display_pos_y: reader.read_u32(ZombieOffset::DisplayPosY).unwrap(),
            row: reader.read_u32(ZombieOffset::Row).unwrap(),
            zombie_type: reader
                .read_u32(ZombieOffset::ZombieType)
                .unwrap()
                .try_into()
                .unwrap(),
            pos_x: reader.read_f32(ZombieOffset::PosX).unwrap(),
            pos_y: reader.read_f32(ZombieOffset::PosY).unwrap(),
            freeze_timer: reader.read_u32(ZombieOffset::FreezeTimer).unwrap(),
            is_hypnotized: reader.read_bool(ZombieOffset::IsHypnotized).unwrap(),
            armor_type: reader
                .read_u32(ZombieOffset::ArmorType)
                .unwrap()
                .try_into()
                .unwrap(),
            health: reader.read_i32(ZombieOffset::Health).unwrap(),
            original_health: reader.read_i32(ZombieOffset::OriginalHealth).unwrap(),
            armor_hp: reader.read_u32(ZombieOffset::ArmorHealth).unwrap(),
            original_armor_hp: reader.read_u32(ZombieOffset::OriginalArmorHealth).unwrap(),
            is_dead: reader.read_bool(ZombieOffset::IsDead).unwrap(),
        }
    }
}
