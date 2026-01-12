use valence_math::DVec3;

use valence_binary::{Decode, Encode, Packet};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct MoveVehicleS2c {
    pub position: DVec3,
    pub yaw: f32,
    pub pitch: f32,
}
