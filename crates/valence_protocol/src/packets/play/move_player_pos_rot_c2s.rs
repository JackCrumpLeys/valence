use valence_math::DVec3;

use crate::Packet;
use crate::movement_flags::MovementFlags;
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct MovePlayerPosRotC2s {
    pub position: DVec3,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: MovementFlags,
}
