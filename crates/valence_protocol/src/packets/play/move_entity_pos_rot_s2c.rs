use crate::movement_flags::MovementFlags;
use crate::ByteAngle;
use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct MoveEntityPosRotS2c {
    pub entity_id: VarInt,
    pub delta: [i16; 3],
    pub yaw: ByteAngle,
    pub pitch: ByteAngle,
    pub flags: MovementFlags,
}
