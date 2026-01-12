use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct ChunkBatchReceivedC2s {
    pub chunks_per_tick: f32,
}
