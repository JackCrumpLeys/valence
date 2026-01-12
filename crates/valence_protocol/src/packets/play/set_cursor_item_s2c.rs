use crate::Packet;
use valence_binary::{Decode, Encode};
use valence_item::ItemStack;

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SetCursorItemS2c {
    item: ItemStack,
}
