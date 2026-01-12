use crate::movement_flags::MovementFlags;
use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct MovePlayerStatusOnlyC2s {
    pub flags: MovementFlags,
}
