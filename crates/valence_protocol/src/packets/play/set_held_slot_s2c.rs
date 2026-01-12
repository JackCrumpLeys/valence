use valence_binary::{Decode, Encode, Packet, VarInt};
#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct SetHeldSlotS2c {
    pub slot: VarInt,
}
