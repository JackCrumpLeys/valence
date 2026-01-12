use std::borrow::Cow;

use valence_binary::{Decode, Encode, Packet, VarInt};
use valence_item::ItemStack;

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct ContainerSetContentS2c<'a> {
    pub window_id: VarInt,
    pub state_id: VarInt,
    pub slots: Cow<'a, [ItemStack]>,
    pub carried_item: Cow<'a, ItemStack>,
}
