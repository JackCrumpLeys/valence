use valence_binary::{Decode, Encode, Packet, VarInt};
#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SetCameraS2c {
    pub entity_id: VarInt,
}
