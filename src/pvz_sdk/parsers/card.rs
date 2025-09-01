use crate::ReaderAt;
use crate::models::Card;
use crate::offsets::CardOffset;
use crate::traits::{ReadEntityError, ReadableEntity};

impl ReadableEntity for Card {
    const SIZE: usize = 80;

    fn read(reader: ReaderAt) -> Result<Self, ReadEntityError> {
        assert_eq!(reader.len(), Self::SIZE);

        Ok(Self {
            display_pos_x: reader.read_u32(CardOffset::DisplayPosX)?,
            display_pos_y: reader.read_u32(CardOffset::DisplayPosY)?,
            selectable_width: reader.read_u32(CardOffset::SelectableWidth)?,
            selectable_height: reader.read_u32(CardOffset::SelectableHeight)?,
            charge: reader.read_u32(CardOffset::Charge)?,
            recharge_goal: reader.read_u32(CardOffset::RechargeGoal)?,
            column: reader.read_u32(CardOffset::Column)?,
            pos_x_offset: reader.read_i32(CardOffset::PosXOffset)?,
            card_type: reader.read_u32(CardOffset::CardType)?.try_into()?,
            selectable: reader.read_bool(CardOffset::Selectable)?,
            recharging: reader.read_bool(CardOffset::Recharging)?,
            usage_count: reader.read_u32(CardOffset::UsageCount)?,
        })
    }
}
