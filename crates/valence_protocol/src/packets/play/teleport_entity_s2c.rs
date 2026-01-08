use valence_math::DVec3;

use crate::packets::play::player_position_s2c::TeleportRelativeFlags;
use crate::{ByteAngle, Decode, Encode, Packet, VarInt};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct TeleportEntityS2c {
    pub entity_id: VarInt,
    pub position: DVec3,
    pub velocity: DVec3,
    pub yaw: ByteAngle,
    pub pitch: ByteAngle,
    pub flags: TeleportRelativeFlags,
    pub on_ground: bool,
}
