use crate::Packet;
use crate::BlockPos;
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct SetDefaultSpawnPositionS2c {
    pub position: BlockPos,
    pub angle: f32,
}
