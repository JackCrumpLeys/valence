use crate::BlockPos;
use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct OpenSignEditorS2c {
    pub location: BlockPos,
    pub is_front_text: bool,
}
