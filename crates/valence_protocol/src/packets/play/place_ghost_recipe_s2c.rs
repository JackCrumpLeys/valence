use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct PlaceGhostRecipeS2c {
    pub window_id: VarInt,
    pub recipe_display: VarInt,
}
