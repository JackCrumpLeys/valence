use valence_binary::{Decode, Encode, Packet, VarInt};

use crate::BlockPos;

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct BlockEntityTagQueryC2s {
    pub transaction_id: VarInt,
    pub position: BlockPos,
}
