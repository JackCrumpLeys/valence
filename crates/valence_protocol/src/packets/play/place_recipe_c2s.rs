use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct PlaceRecipeC2s {
    pub window_id: i8,
    pub recipe: VarInt,
    pub make_all: bool,
}
