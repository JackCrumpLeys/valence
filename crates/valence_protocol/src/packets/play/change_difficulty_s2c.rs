use crate::Difficulty;
use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, Packet)]
pub struct ChangeDifficultyS2c {
    pub difficulty: Difficulty,
    pub locked: bool,
}
