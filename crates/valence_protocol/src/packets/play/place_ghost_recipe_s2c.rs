use valence_binary::{Decode, Encode, VarInt};

use crate::Packet;

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct PlaceGhostRecipeS2c {
    pub window_id: VarInt,
    pub recipe_display: VarInt,
}
