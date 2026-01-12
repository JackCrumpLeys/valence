use crate::Hand;
use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct OpenBookS2c {
    pub hand: Hand,
}
