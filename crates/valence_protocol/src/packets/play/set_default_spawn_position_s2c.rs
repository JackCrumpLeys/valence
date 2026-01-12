use crate::BlockPos;
use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct SetDefaultSpawnPositionS2c {
    pub position: BlockPos,
    pub angle: f32,
}
