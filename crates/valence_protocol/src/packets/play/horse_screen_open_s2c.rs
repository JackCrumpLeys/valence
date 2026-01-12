use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct HorseScreenOpenS2c {
    pub window_id: u8,
    pub slot_count: VarInt,
    pub entity_id: i32,
}
