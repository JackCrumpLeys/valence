use std::borrow::Cow;
use std::io::Write;

use anyhow::{bail, ensure};
use valence_ident::Ident;

use crate::{packet_id, Decode, Encode, ItemStack, Packet, VarInt};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(id = packet_id::PLAY_UPDATE_RECIPES_S2C)]
pub struct UpdateRecipesS2c<'a> {
    pub recipes: Vec<Recipe<'a>>,
}

#[derive(Clone, Debug)]
pub struct Recipe<'a> {
    pub kind: Ident<Cow<'a, str>>,
    pub recipe_id: Ident<Cow<'a, str>>,
    pub data: RecipeData<'a>,
}

impl Encode for Recipe<'_> {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        self.kind.encode(&mut w)?;
        self.recipe_id.encode(&mut w)?;
        self.data.encode(w)
    }
}

impl<'a> Decode<'a> for Recipe<'a> {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        let kind = Ident::decode(r)?;
        let recipe_id = Ident::decode(r)?;
        let data = RecipeData::decode_with_kind(r, kind.as_str())?;
        Ok(Self {
            kind,
            recipe_id,
            data,
        })
    }
}

#[derive(Clone, Debug)]
pub enum RecipeData<'a> {
    CraftingShaped(CraftingShapedData<'a>),
    CraftingShapeless(CraftingShapelessData<'a>),
    CraftingSpecialArmordye(CraftingSpecialData),
    CraftingSpecialBookcloning(CraftingSpecialData),
    CraftingSpecialMapcloning(CraftingSpecialData),
    CraftingSpecialMapextending(CraftingSpecialData),
    CraftingSpecialFireworkRocket(CraftingSpecialData),
    CraftingSpecialFireworkStar(CraftingSpecialData),
    CraftingSpecialFireworkStarFade(CraftingSpecialData),
    CraftingSpecialTippedarrow(CraftingSpecialData),
    CraftingSpecialBannerduplicate(CraftingSpecialData),
    CraftingSpecialShielddecoration(CraftingSpecialData),
    CraftingSpecialShulkerboxcoloring(CraftingSpecialData),
    CraftingSpecialSuspiciousStew(CraftingSpecialData),
    CraftingSpecialRepairitem(CraftingSpecialData),
    CraftingDecoratedPot(CraftingSpecialData),
    Smelting(CookingData<'a>),
    Blasting(CookingData<'a>),
    Smoking(CookingData<'a>),
    CampfireCooking(CookingData<'a>),
    Stonecutting(StonecuttingData<'a>),
    SmithingTransform(SmithingTransformData),
    SmithingTrim(SmithingTrimData),
}

impl Encode for RecipeData<'_> {
    fn encode(&self, w: impl Write) -> anyhow::Result<()> {
        match self {
            RecipeData::CraftingShaped(d) => d.encode(w),
            RecipeData::CraftingShapeless(d) => d.encode(w),
            RecipeData::CraftingSpecialArmordye(d)
            | RecipeData::CraftingSpecialBookcloning(d)
            | RecipeData::CraftingSpecialMapcloning(d)
            | RecipeData::CraftingSpecialMapextending(d)
            | RecipeData::CraftingSpecialFireworkRocket(d)
            | RecipeData::CraftingSpecialFireworkStar(d)
            | RecipeData::CraftingSpecialFireworkStarFade(d)
            | RecipeData::CraftingSpecialTippedarrow(d)
            | RecipeData::CraftingSpecialBannerduplicate(d)
            | RecipeData::CraftingSpecialShielddecoration(d)
            | RecipeData::CraftingSpecialShulkerboxcoloring(d)
            | RecipeData::CraftingSpecialSuspiciousStew(d)
            | RecipeData::CraftingSpecialRepairitem(d)
            | RecipeData::CraftingDecoratedPot(d) => d.encode(w),
            RecipeData::Smelting(d)
            | RecipeData::Blasting(d)
            | RecipeData::Smoking(d)
            | RecipeData::CampfireCooking(d) => d.encode(w),
            RecipeData::Stonecutting(d) => d.encode(w),
            RecipeData::SmithingTransform(d) => d.encode(w),
            RecipeData::SmithingTrim(d) => d.encode(w),
        }
    }
}

impl<'a> RecipeData<'a> {
    fn decode_with_kind(r: &mut &'a [u8], kind: &str) -> anyhow::Result<Self> {
        Ok(match kind {
            "minecraft:crafting_shaped" => Self::CraftingShaped(Decode::decode(r)?),
            "minecraft:crafting_shapeless" => Self::CraftingShapeless(Decode::decode(r)?),
            "minecraft:crafting_special_armordye" => {
                Self::CraftingSpecialArmordye(Decode::decode(r)?)
            }
            "minecraft:crafting_special_bookcloning" => {
                Self::CraftingSpecialBookcloning(Decode::decode(r)?)
            }
            "minecraft:crafting_special_mapcloning" => {
                Self::CraftingSpecialMapcloning(Decode::decode(r)?)
            }
            "minecraft:crafting_special_mapextending" => {
                Self::CraftingSpecialMapextending(Decode::decode(r)?)
            }
            "minecraft:crafting_special_firework_rocket" => {
                Self::CraftingSpecialFireworkRocket(Decode::decode(r)?)
            }
            "minecraft:crafting_special_firework_star" => {
                Self::CraftingSpecialFireworkStar(Decode::decode(r)?)
            }
            "minecraft:crafting_special_firework_star_fade" => {
                Self::CraftingSpecialFireworkStarFade(Decode::decode(r)?)
            }
            "minecraft:crafting_special_tippedarrow" => {
                Self::CraftingSpecialTippedarrow(Decode::decode(r)?)
            }
            "minecraft:crafting_special_bannerduplicate" => {
                Self::CraftingSpecialBannerduplicate(Decode::decode(r)?)
            }
            "minecraft:crafting_special_shielddecoration" => {
                Self::CraftingSpecialShielddecoration(Decode::decode(r)?)
            }
            "minecraft:crafting_special_shulkerboxcoloring" => {
                Self::CraftingSpecialShulkerboxcoloring(Decode::decode(r)?)
            }
            "minecraft:crafting_special_suspiciousstew" => {
                Self::CraftingSpecialSuspiciousStew(Decode::decode(r)?)
            }
            "minecraft:crafting_special_repairitem" => {
                Self::CraftingSpecialRepairitem(Decode::decode(r)?)
            }
            "minecraft:crafting_decorated_pot" => Self::CraftingDecoratedPot(Decode::decode(r)?),
            "minecraft:smelting" => Self::Smelting(Decode::decode(r)?),
            "minecraft:blasting" => Self::Blasting(Decode::decode(r)?),
            "minecraft:smoking" => Self::Smoking(Decode::decode(r)?),
            "minecraft:campfire_cooking" => Self::CampfireCooking(Decode::decode(r)?),
            "minecraft:stonecutting" => Self::Stonecutting(Decode::decode(r)?),
            "minecraft:smithing_transform" => Self::SmithingTransform(Decode::decode(r)?),
            "minecraft:smithing_trim" => Self::SmithingTrim(Decode::decode(r)?),
            _ => bail!("unknown recipe kind {kind}"),
        })
    }
}

/// Helper to encode an `ItemStack` as an Ingredient (Count + Slots).
/// Since `valence` uses `ItemStack` which represents a single item, we encode
/// it as an ingredient with 1 option if not empty, or 0 options if empty.
fn encode_ingredient(item: &ItemStack, mut w: impl Write) -> anyhow::Result<()> {
    if item.is_empty() {
        VarInt(0).encode(w)
    } else {
        VarInt(1).encode(&mut w)?;
        item.encode(w)
    }
}

/// Helper to decode an Ingredient into an `ItemStack`.
/// Takes the first slot if multiple are present.
fn decode_ingredient(r: &mut &[u8]) -> anyhow::Result<ItemStack> {
    let count = VarInt::decode(r)?.0;
    if count == 0 {
        Ok(ItemStack::EMPTY)
    } else {
        let item = ItemStack::decode(r)?;
        // Consume remaining options if any, ignoring them.
        for _ in 1..count {
            let _ = ItemStack::decode(r)?;
        }
        Ok(item)
    }
}

#[derive(Clone, Debug)]
pub struct CraftingShapedData<'a> {
    pub group: Cow<'a, str>,
    pub category: CraftingBookCategory,
    pub width: VarInt,
    pub height: VarInt,
    /// Length must be width * height.
    pub ingredients: Cow<'a, [ItemStack]>,
    pub result: ItemStack,
    pub show_notification: bool,
}

impl Encode for CraftingShapedData<'_> {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        let Self {
            width,
            height,
            group,
            category,
            ingredients,
            result,
            show_notification,
        } = self;

        width.encode(&mut w)?;
        height.encode(&mut w)?;
        group.encode(&mut w)?;
        category.encode(&mut w)?;

        let len = (width.0 * height.0) as usize;

        ensure!(
            len == ingredients.len(),
            "number of ingredients in shaped recipe must be equal to width * height"
        );

        for ingr in ingredients.as_ref() {
            encode_ingredient(ingr, &mut w)?;
        }

        result.encode(&mut w)?;
        show_notification.encode(w)
    }
}

impl<'a> Decode<'a> for CraftingShapedData<'a> {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        let width = VarInt::decode(r)?;
        let height = VarInt::decode(r)?;
        let group = Decode::decode(r)?;
        let category = Decode::decode(r)?;

        let len = (width.0 * height.0) as usize;
        let mut ingredients = Vec::with_capacity(len);
        for _ in 0..len {
            ingredients.push(decode_ingredient(r)?);
        }

        let result = Decode::decode(r)?;
        let show_notification = Decode::decode(r)?;

        Ok(Self {
            group,
            category,
            width,
            height,
            ingredients: Cow::Owned(ingredients),
            result,
            show_notification,
        })
    }
}

#[derive(Clone, Debug)]
pub struct CraftingShapelessData<'a> {
    pub group: Cow<'a, str>,
    pub category: CraftingBookCategory,
    pub ingredients: Cow<'a, [ItemStack]>,
    pub result: ItemStack,
    pub show_notification: bool,
}

impl Encode for CraftingShapelessData<'_> {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        self.group.encode(&mut w)?;
        self.category.encode(&mut w)?;
        VarInt(self.ingredients.len() as i32).encode(&mut w)?;
        for ingr in self.ingredients.as_ref() {
            encode_ingredient(ingr, &mut w)?;
        }
        self.result.encode(&mut w)?;
        self.show_notification.encode(w)
    }
}

impl<'a> Decode<'a> for CraftingShapelessData<'a> {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        let group = Decode::decode(r)?;
        let category = Decode::decode(r)?;
        let count = VarInt::decode(r)?.0;
        let mut ingredients = Vec::with_capacity(count as usize);
        for _ in 0..count {
            ingredients.push(decode_ingredient(r)?);
        }
        let result = Decode::decode(r)?;
        let show_notification = Decode::decode(r)?;

        Ok(Self {
            group,
            category,
            ingredients: Cow::Owned(ingredients),
            result,
            show_notification,
        })
    }
}

#[derive(Clone, Debug, Encode, Decode)]
pub struct CraftingSpecialData {
    pub category: CraftingBookCategory,
}

#[derive(Clone, Debug)]
pub struct CookingData<'a> {
    pub group: Cow<'a, str>,
    pub category: CookingBookCategory,
    pub ingredient: ItemStack,
    pub result: ItemStack,
    pub experience: f32,
    pub cooking_time: VarInt,
}

impl Encode for CookingData<'_> {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        self.group.encode(&mut w)?;
        self.category.encode(&mut w)?;
        encode_ingredient(&self.ingredient, &mut w)?;
        self.result.encode(&mut w)?;
        self.experience.encode(&mut w)?;
        self.cooking_time.encode(w)
    }
}

impl<'a> Decode<'a> for CookingData<'a> {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        Ok(Self {
            group: Decode::decode(r)?,
            category: Decode::decode(r)?,
            ingredient: decode_ingredient(r)?,
            result: Decode::decode(r)?,
            experience: Decode::decode(r)?,
            cooking_time: Decode::decode(r)?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct StonecuttingData<'a> {
    pub group: Cow<'a, str>,
    pub ingredient: ItemStack,
    pub result: ItemStack,
}

impl Encode for StonecuttingData<'_> {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        self.group.encode(&mut w)?;
        encode_ingredient(&self.ingredient, &mut w)?;
        self.result.encode(w)
    }
}

impl<'a> Decode<'a> for StonecuttingData<'a> {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        Ok(Self {
            group: Decode::decode(r)?,
            ingredient: decode_ingredient(r)?,
            result: Decode::decode(r)?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct SmithingTransformData {
    pub template: ItemStack,
    pub base: ItemStack,
    pub addition: ItemStack,
    pub result: ItemStack,
}

impl Encode for SmithingTransformData {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        encode_ingredient(&self.template, &mut w)?;
        encode_ingredient(&self.base, &mut w)?;
        encode_ingredient(&self.addition, &mut w)?;
        self.result.encode(w)
    }
}

impl<'a> Decode<'a> for SmithingTransformData {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        Ok(Self {
            template: decode_ingredient(r)?,
            base: decode_ingredient(r)?,
            addition: decode_ingredient(r)?,
            result: Decode::decode(r)?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct SmithingTrimData {
    pub template: ItemStack,
    pub base: ItemStack,
    pub addition: ItemStack,
}

impl Encode for SmithingTrimData {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        encode_ingredient(&self.template, &mut w)?;
        encode_ingredient(&self.base, &mut w)?;
        encode_ingredient(&self.addition, &mut w)?;
        Ok(())
    }
}

impl<'a> Decode<'a> for SmithingTrimData {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        Ok(Self {
            template: decode_ingredient(r)?,
            base: decode_ingredient(r)?,
            addition: decode_ingredient(r)?,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode)]
pub enum CraftingBookCategory {
    Building,
    Redstone,
    Equipment,
    Misc,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode)]
pub enum CookingBookCategory {
    Food,
    Blocks,
    Misc,
}
