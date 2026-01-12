use valence_binary::{Decode, Encode, Packet, VarInt};
#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct RecipeBookSeenRecipeC2s {
    pub recipe_id: VarInt,
}
