use crate::BlockPos;
use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct BlockDestructionS2c {
    pub entity_id: VarInt,
    pub position: BlockPos,
    pub destroy_stage: u8,
}
