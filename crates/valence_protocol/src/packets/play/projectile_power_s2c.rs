use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};
#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct ProjectilePowerS2c {
    pub entity_id: VarInt,
    pub power: f64,
}
