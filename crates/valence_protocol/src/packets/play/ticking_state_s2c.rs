use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct TickingStateS2c {
    pub tick_rate: f32,
    pub is_frozen: bool,
}
