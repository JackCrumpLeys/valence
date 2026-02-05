use std::borrow::Cow;

use valence_binary::{
    registry_id::{PlaceholderDynamicRegistryItem, RegistryId},
    Decode, Encode, IDSet, VarInt,
};
use valence_ident::Ident;
use valence_item::{ItemKind, ItemStack};

use crate::Packet;

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct UpdateRecipesS2c<'a> {
    pub property_sets: Vec<PropertySet<'a>>,
    pub stonecutter_recipes: Vec<StonecutterRecipe<'a>>,
}

#[derive(Clone, Debug, Encode, Decode)]
pub struct PropertySet<'a> {
    pub id: Ident<Cow<'a, str>>,
    pub items: Vec<VarInt>,
}

#[derive(Clone, Debug, Encode, Decode)]
pub struct StonecutterRecipe<'a> {
    pub ingredients: IDSet<ItemKind>,
    pub result: SlotDisplay<'a>,
}

#[derive(Clone, Debug, Encode, Decode)]
pub enum SlotDisplay<'a> {
    Empty,
    AnyFuel,
    Item(RegistryId<ItemKind>),
    ItemStack(Box<ItemStack>),
    Tag(Ident<Cow<'a, str>>),
    SmithingTrim {
        base: Box<SlotDisplay<'a>>,
        material: Box<SlotDisplay<'a>>,
        pattern: RegistryId<PlaceholderDynamicRegistryItem>, // ID in trim_pattern registry
    },
    WithRemainder {
        ingredient: Box<SlotDisplay<'a>>,
        remainder: Box<SlotDisplay<'a>>,
    },
    Composite(Vec<SlotDisplay<'a>>),
}
