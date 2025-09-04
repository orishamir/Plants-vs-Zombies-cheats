use crate::Popcapgame;
use crate::entities::{Slot, Slots};
use crate::offsets::{SlotOffset, SlotsOffset};
use crate::readers::OffsetReader;
use crate::readers::offset_reader::OffsetReaderError;
use crate::readers::traits::{ReadEntityError, ReadableEntity};

const SLOT_MEMORY_SIZE: usize = 80;

impl ReadableEntity for Slots {
    fn read(game: &Popcapgame) -> Result<Self, ReadEntityError> {
        let base_addr = game.read_ptr_chain(&[0x331C50, 0x320, 0x18, 0x0, 0x8, 0x15c, 0], true)?;
        let slots_reader = OffsetReader::new(base_addr, game);

        let slots_count = slots_reader.read_u32(SlotsOffset::Count)?;

        let slots = (0..slots_count)
            .filter_map(|i| {
                Self::read_slot(
                    base_addr + SlotsOffset::Array as usize + i as usize * SLOT_MEMORY_SIZE,
                    game,
                )
                .ok()
            })
            .collect::<Vec<Slot>>();

        Ok(Self {
            pos_x: slots_reader.read_i32(SlotsOffset::PosX)?,
            pos_y: slots_reader.read_i32(SlotsOffset::PosY)?,
            width: slots_reader.read_u32(SlotsOffset::Width)?,
            height: slots_reader.read_u32(SlotsOffset::Height)?,
            slots_count,
            slots,
        })
    }
}

impl Slots {
    fn read_slot(addr: usize, game: &Popcapgame) -> Result<Slot, OffsetReaderError> {
        let reader = OffsetReader::new(addr, game);

        Ok(Slot {
            addr,
            display_pos_x: reader.read_u32(SlotOffset::DisplayPosX)?,
            display_pos_y: reader.read_u32(SlotOffset::DisplayPosY)?,
            selectable_width: reader.read_u32(SlotOffset::SelectableWidth)?,
            selectable_height: reader.read_u32(SlotOffset::SelectableHeight)?,
            charge: reader.read_u32(SlotOffset::Charge)?,
            recharge_goal: reader.read_u32(SlotOffset::RechargeGoal)?,
            column: reader.read_u32(SlotOffset::Column)?,
            pos_x_offset: reader.read_i32(SlotOffset::PosXOffset)?,
            card_type: reader.read_u32(SlotOffset::CardType)?.try_into().unwrap(),
            selectable: reader.read_bool(SlotOffset::Selectable)?,
            recharging: reader.read_bool(SlotOffset::Recharging)?,
            usage_count: reader.read_u32(SlotOffset::UsageCount)?,
        })
    }
}
