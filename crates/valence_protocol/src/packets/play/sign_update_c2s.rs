use crate::BlockPos;
use crate::Packet;
use valence_binary::{Bounded, Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct SignUpdateC2s<'a> {
    pub position: BlockPos,
    pub is_front_text: bool,
    pub lines: [Bounded<&'a str, 384>; 4],
}
