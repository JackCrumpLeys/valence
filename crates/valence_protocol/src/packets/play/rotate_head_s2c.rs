use crate::ByteAngle;
use valence_binary::{Decode, Encode, Packet, VarInt};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct RotateHeadS2c {
    pub entity_id: VarInt,
    pub head_yaw: ByteAngle,
}
