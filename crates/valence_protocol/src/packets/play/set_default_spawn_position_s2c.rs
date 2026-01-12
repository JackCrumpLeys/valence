use crate::BlockPos;
use valence_binary::{Decode, Encode, Packet};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct SetDefaultSpawnPositionS2c {
    pub position: BlockPos,
    pub angle: f32,
}
