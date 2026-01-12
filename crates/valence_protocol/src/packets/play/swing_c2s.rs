use crate::Packet;
use crate::Hand;
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct SwingC2s {
    pub hand: Hand,
}
