use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct ChunkBatchFinishedS2c {
    pub batch_size: VarInt,
}
