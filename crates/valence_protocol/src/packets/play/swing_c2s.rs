use crate::Hand;
use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct SwingC2s {
    pub hand: Hand,
}
