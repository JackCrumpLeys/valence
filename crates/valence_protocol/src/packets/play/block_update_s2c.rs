use crate::Packet;
use crate::{BlockPos, BlockState};
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct BlockUpdateS2c {
    pub position: BlockPos,
    pub block_id: BlockState,
}
