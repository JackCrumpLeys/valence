use crate::Hand;
use valence_binary::{Decode, Encode, Packet};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct OpenBookS2c {
    pub hand: Hand,
}
