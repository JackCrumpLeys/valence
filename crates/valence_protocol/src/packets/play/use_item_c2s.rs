use crate::Hand;
use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct UseItemC2s {
    pub hand: Hand,
    pub sequence: VarInt,
    pub yaw: f32,
    pub pitch: f32,
}
