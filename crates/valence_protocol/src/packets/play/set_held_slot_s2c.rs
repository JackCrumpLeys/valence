use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};
#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct SetHeldSlotS2c {
    pub slot: VarInt,
}
