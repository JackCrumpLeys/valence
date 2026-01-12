use std::borrow::Cow;

use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};
use valence_item::ItemStack;

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SetPlayerInventoryS2c<'a> {
    pub slot: VarInt,
    pub slot_data: Cow<'a, ItemStack>,
}
