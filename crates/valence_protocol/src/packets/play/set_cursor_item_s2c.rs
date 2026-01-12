use valence_binary::{Decode, Encode, Packet};
use valence_item::ItemStack;

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SetCursorItemS2c {
    item: ItemStack,
}
