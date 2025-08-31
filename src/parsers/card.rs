use crate::ReaderAt;
use crate::models::Card;
use crate::offsets::CardOffset;
use crate::traits::ReadableEntity;

impl ReadableEntity for Card {
    const SIZE: usize = 80;

    fn read(reader: ReaderAt) -> Self {
        assert_eq!(reader.len(), Self::SIZE);

        Self {
            display_pos_x: reader.read_u32(CardOffset::DisplayPosX).unwrap(),
            display_pos_y: reader.read_u32(CardOffset::DisplayPosY).unwrap(),
            selectable_width: reader.read_u32(CardOffset::SelectableWidth).unwrap(),
            selectable_height: reader.read_u32(CardOffset::SelectableHeight).unwrap(),
            charge: reader.read_u32(CardOffset::Charge).unwrap(),
            recharge_goal: reader.read_u32(CardOffset::RechargeGoal).unwrap(),
            column: reader.read_u32(CardOffset::Column).unwrap(),
            pos_x_offset: reader.read_i32(CardOffset::PosXOffset).unwrap(),
            card_type: reader.read_u32(CardOffset::CardType).unwrap().into(),
            selectable: reader.read_bool(CardOffset::Selectable).unwrap(),
            recharging: reader.read_bool(CardOffset::Recharging).unwrap(),
            usage_count: reader.read_u32(CardOffset::UsageCount).unwrap(),
        }
    }
}
