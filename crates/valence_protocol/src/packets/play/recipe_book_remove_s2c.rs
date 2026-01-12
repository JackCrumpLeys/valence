use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};
#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct RecipeBookRemoveS2c {
    pub recipes: Vec<VarInt>,
}
