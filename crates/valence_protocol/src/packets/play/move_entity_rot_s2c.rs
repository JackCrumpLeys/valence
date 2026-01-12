use crate::ByteAngle;
use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct MoveEntityRotS2c {
    pub entity_id: VarInt,
    pub yaw: ByteAngle,
    pub pitch: ByteAngle,
    pub on_ground: bool,
}
