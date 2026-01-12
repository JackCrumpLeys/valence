use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};
#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct SelectTradeC2s {
    pub selected_slot: VarInt,
}
