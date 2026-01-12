use crate::Difficulty;
use valence_binary::{Decode, Encode, Packet};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, Packet)]
pub struct ChangeDifficultyS2c {
    pub difficulty: Difficulty,
    pub locked: bool,
}
