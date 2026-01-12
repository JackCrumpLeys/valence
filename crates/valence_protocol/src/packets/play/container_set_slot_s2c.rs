use std::borrow::Cow;

use valence_binary::{Decode, Encode, Packet, VarInt};
use valence_item::ItemStack;

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct ContainerSetSlotS2c<'a> {
    pub window_id: VarInt,
    pub state_id: VarInt,
    pub slot_idx: i16,
    pub slot_data: Cow<'a, ItemStack>,
}
