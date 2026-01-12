use crate::Packet;
use crate::BlockPos;
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct PickItemFromBlockC2s {
    pub block_position: BlockPos,
    pub include_data: bool,
}
