use valence_nbt::Compound;

use valence_binary::{Decode, Encode, Packet, VarInt};
#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct TagQueryS2c {
    pub transaction_id: VarInt,
    pub nbt: Compound,
}
