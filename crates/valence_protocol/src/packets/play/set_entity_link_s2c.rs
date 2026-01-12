use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct SetEntityLinkS2c {
    pub attached_entity_id: i32,
    pub holding_entity_id: i32,
}
