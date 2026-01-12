use crate::Packet;
use crate::Difficulty;
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, Packet)]
pub struct ChangeDifficultyC2s {
    pub difficulty: Difficulty,
}
