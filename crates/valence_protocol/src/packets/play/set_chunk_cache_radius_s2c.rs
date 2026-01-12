use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};
#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct SetChunkCacheRadiusS2c {
    pub view_distance: VarInt,
}
