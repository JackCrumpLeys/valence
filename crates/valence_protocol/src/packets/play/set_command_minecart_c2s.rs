use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};
#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct SetCommandMinecartC2s<'a> {
    pub entity_id: VarInt,
    pub command: &'a str,
    pub track_output: bool,
}
