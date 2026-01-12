use std::borrow::Cow;

use valence_binary::{Decode, Encode, Packet, VarInt};
use valence_item::ItemStack;

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SetPlayerInventoryS2c<'a> {
    pub slot: VarInt,
    pub slot_data: Cow<'a, ItemStack>,
}
