use std::borrow::Cow;
use std::io::Write;

use valence_generated::attributes::{EntityAttribute, EntityAttributeOperation};
pub use valence_generated::item::ItemKind;
use valence_generated::registry_id::RegistryId;
pub use valence_generated::sound::Sound;
use valence_ident::Ident;
use valence_nbt::Compound;
use valence_text::color::RgbColor;
use valence_text::Text;

use crate::sound::SoundId;
use crate::{Decode, Encode, IDSet, VarInt};

const NUM_ITEM_COMPONENTS: usize = 96;

/// A stack of items in an inventory.
#[derive(Clone, PartialEq, Debug)]
pub struct ItemStack {
    pub item: ItemKind,
    pub count: i8,
    components: [Option<Box<ItemComponent>>; NUM_ITEM_COMPONENTS],
}

impl Default for ItemStack {
    fn default() -> Self {
        ItemStack::EMPTY
    }
}

type StrIdent = Ident<String>;

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum ItemComponent {
    /// Customizable data that doesn't fit any specific component.
    CustomData {
        /// Always a Compound Tag.
        data: Compound,
    },
    /// Maximum stack size for the item.
    MaxStackSize {
        /// Ranges from 1 to 99.
        max_stack_size: VarInt,
    },
    /// The maximum damage the item can take before breaking.
    MaxDamage {
        max_damage: VarInt,
    },
    /// The current damage of the item.
    Damage {
        damage: VarInt,
    },
    /// Marks the item as unbreakable.
    Unbreakable,
    /// Item's custom name. Normally shown in italic, and changeable at an
    /// anvil.
    CustomName {
        name: Text,
    },
    /// Override for the item's default name. Shown when the item has no custom
    /// name.
    ItemName {
        name: Text,
    },
    /// Item's model.
    ItemModel {
        /// The model identifier.
        model: StrIdent,
    },
    /// Item's lore.
    Lore {
        /// The lore lines.
        lines: Vec<Text>,
    },
    /// Item's rarity. This affects the default color of the item's name.
    Rarity {
        rarity: Rarity,
    },
    /// The enchantments of the item.
    Enchantments {
        /// The enchantments. (The ID of the enchantment in the enchantment
        /// registry, The level of the enchantment)
        enchantments: Vec<(VarInt, VarInt)>,
    },
    /// List of blocks this block can be placed on when in adventure mode.
    CanPlaceOn {
        /// The block predicates.
        block_predicates: Vec<BlockPredicate>,
    },
    /// List of blocks this item can break when in adventure mode.
    CanBreak {
        /// The block predicates.
        block_predicates: Vec<BlockPredicate>,
    },
    /// The attribute modifiers of the item.
    AttributeModifiers {
        /// The attributes.
        attributes: Vec<ItemAttribute>,
        /// Whether the modifiers should be shown on the item's tooltip.
        show_in_tooltip: bool,
    },
    /// Value for the item predicate when using custom item models.
    CustomModelData {
        value: VarInt,
    },
    /// Allows you to hide all or parts of the item tooltip.
    TooltipDisplay {
        /// Whether to hide the tooltip entirely.
        hide_tooltip: bool,
        // The IDs of data components in the minecraft:data_component_type registry to hide.
        hidden_components: Vec<VarInt>,
    },
    /// Accumulated anvil usage cost.
    RepairCost {
        cost: VarInt,
    },
    /// Marks the item as non-interactive on the creative inventory (the first 5
    /// rows of items).
    CreativeSlotLock,
    /// Overrides the item glint resulted from enchantments.
    EnchantmentGlintOverride {
        has_glint: bool,
    },
    /// Marks the projectile as intangible (cannot be picked-up).
    IntangibleProjectile,
    /// Makes the item restore players hunger when eaten.
    Food {
        /// Non-negative.
        nutrition: VarInt,
        /// How much saturation will be given after consuming the item.
        saturation_modifier: f32,
        /// Whether the item can always be eaten, even at full hunger.
        can_always_eat: bool,
    },
    /// Makes the item consumable.
    Consumable {
        /// How long it takes to consume the item.
        consume_seconds: f32,
        /// The animation type.
        animation: ConsumableAnimation,
        /// The sound event.
        sound: SoundId,
        /// Whether the item has consume particles.
        has_consume_particles: bool,
        /// Number of elements in the following array.
        number_of_effects: VarInt,
        /// The effects.
        effects: Vec<u8>, // TODO
    },
    /// This specifies the item produced after using the current item.
    UseRemainder {
        /// The remainder item.
        // FIX: remainder: ItemStack,
        remainder: u8,
    },
    /// Cooldown to apply on use of the item.
    UseCooldown {
        /// The cooldown duration in seconds.
        seconds: f32,
        /// The cooldown group identifier.
        cooldown_group: Option<StrIdent>,
    },
    /// Marks this item as damage resistant.
    DamageResistant {
        /// Tag specifying damage types the item is immune to. Not prefixed by
        /// '#'.
        types: StrIdent,
    },
    /// Alters the speed at which this item breaks certain blocks.
    Tool {
        /// The rules.
        rules: Vec<ToolRule>,
        mining_speed: f32,
        damage_per_block: VarInt,
    },
    /// Item treated as a weapon.
    Weapon {
        /// Damage per Attack.
        damage: VarInt,
        /// How long blocking will be disabled after an attack.
        disable_blocking_for_secs: f32,
    },
    /// Allows the item to be enchanted by an enchanting table.
    Enchantable {
        /// Opaque internal value controlling how expensive enchantments may be
        /// offered.
        value: VarInt,
    },
    /// Allows the item to be equipped by the player.
    Equippable {
        /// The slot type.
        slot: EquipSlot,
        /// The equip sound event.
        equip_sound: SoundId,
        /// The model identifier.
        model: Option<String>,
        /// The camera overlay identifier.
        camera_overlay: Option<String>,
        /// Whether the item has allowed entities.
        /// The allowed entities.
        allowed_entities: IDSet,
        /// Whether the item is dispensable.
        dispensable: bool,
        /// Whether the item is swappable.
        swappable: bool,
        /// Whether the item takes damage on hurt.
        damage_on_hurt: bool,
    },
    /// Items that can be combined with this item in an anvil to repair it.
    Repairable {
        /// The items.
        items: IDSet,
    },
    /// Makes the item function like elytra.
    Glider,
    /// Custom textures for the item tooltip.
    TooltipStyle {
        /// The style identifier.
        style: StrIdent,
    },
    /// Makes the item function like a totem of undying.
    DeathProtection {
        /// The effects.
        effects: Vec<u64>, // TODO
    },
    BlocksAttacks, // marked as TODO on minecraft.wiki
    /// The enchantments stored in this enchanted book.
    StoredEnchantments {
        /// The enchantments. The first element is the enchantment ID, the
        /// second is the level.
        enchantments: Vec<(RegistryId, VarInt)>,
        show_in_tooltip: bool,
    },
    /// Color of dyed leather armor.
    DyedColor {
        /// The RGB components of the color, encoded as an integer.
        color: RgbColor,
        /// Whether the armor's color should be shown on the item's tooltip.
        show_in_tooltip: bool,
    },
    /// Color of the markings on the map item model.
    MapColor {
        /// The RGB components of the color, encoded as an integer.
        color: RgbColor,
    },
    /// The ID of the map.
    MapId {
        id: VarInt,
    },
    /// Icons present on a map.
    MapDecorations {
        /// Always a Compound Tag.
        data: Compound,
    },
    /// Used internally by the client when expanding or locking a map. Display
    /// extra information on the item's tooltip when the component is present.
    MapPostProcessing {
        /// Type of post processing.
        type_: MapPostProcessingType,
    },
    /// Projectiles loaded into a charged crossbow.
    ChargedProjectiles {
        /// The projectiles.
        // FIX: projectiles: Vec<ItemStack>,
        projectiles: Vec<u8>,
    },
    /// Contents of a bundle.
    BundleContents {
        /// The items.
        // FIX: items: Vec<ItemStack>,
        items: Vec<u8>,
    },
    /// Visual and effects of a potion item.
    PotionContents {
        /// The ID of the potion type in the potion registry.
        potion_id: Option<RegistryId>,
        /// The RGB components of the color, encoded as an integer.
        custom_color: Option<RgbColor>,
        /// Any custom effects the potion might have.
        custom_effects: Vec<(
            VarInt,
            VarInt,
            bool,
            bool,
            bool,
            bool,
            Option<(VarInt, VarInt, bool, bool, bool, bool)>,
        )>,
        /// Custom name for the potion.
        custom_name: String,
    },
    // description marked as TODO on minecraft.wiki
    PotionDurationScale {
        effects_multiplier: f32,
    },
    /// Effects granted by a suspicious stew.
    SuspiciousStewEffects {
        /// Number of elements in the following array.
        number_of_effects: VarInt,
        /// The effects.
        effects: Vec<(VarInt, VarInt)>,
    },
    /// Content of a writable book.
    WritableBookContent {
        /// Number of elements in the following array.
        number_of_pages: VarInt,
        /// The pages.
        pages: Vec<(String, bool, Option<String>)>,
    },
    /// Content of a written and signed book.
    WrittenBookContent {
        /// The raw title of the book.
        raw_title: String,
        /// Whether the title has been filtered.
        has_filtered_title: bool,
        /// The title after going through chat filters. Only present if Has
        /// Filtered Title is true.
        filtered_title: Option<String>,
        /// The author of the book.
        author: String,
        /// The generation of the book.
        generation: VarInt,
        /// Number of elements in the following array.
        number_of_pages: VarInt,
        /// The pages.
        pages: Vec<(String, bool, Option<String>)>,
        /// Whether entity selectors have already been resolved.
        resolved: bool,
    },
    /// Armor's trim pattern and color.
    Trim {
        /// ID in the `minecraft:trim_material` registry, or an inline
        /// definition.
        trim_material: String,
        /// ID in the `minecraft:trim_pattern` registry, or an inline
        /// definition.
        trim_pattern: String,
        /// Whether the trim information should be shown on the item's tooltip.
        show_in_tooltip: bool,
    },
    /// State of the debug stick.
    DebugStickState {
        /// States of previously interacted blocks. Always a Compound Tag.
        data: Compound,
    },
    /// Data for the entity to be created from this item.
    EntityData {
        /// Always a Compound Tag.
        data: Compound,
    },
    /// Data of the entity contained in this bucket.
    BucketEntityData {
        /// Always a Compound Tag.
        data: Compound,
    },
    /// Data of the block entity to be created from this item.
    BlockEntityData {
        /// Always a Compound Tag.
        data: Compound,
    },
    /// The sound played when using a goat horn.
    Instrument {
        /// ID in the `minecraft:instrument` registry, or an inline definition.
        instrument: String,
    },
    /// marked as TODO on minecraft.wiki
    ProvidesTrimMaterial,
    /// Amplifier for the effect of an ominous bottle.
    OminousBottleAmplifier {
        /// Between 0 and 4.
        amplifier: VarInt,
    },
    /// The song this item will play when inserted into a jukebox.
    JukeboxPlayable {
        /// Whether the jukebox song is specified directly, or just referenced
        /// by name.
        direct_mode: bool,
        /// The name of the jukebox song in its respective registry. Only
        /// present if Direct Mode is false.
        jukebox_song_name: Option<String>,
        /// ID in the `minecraft:jukebox_song` registry. Only present if Direct
        /// Mode is true.
        jukebox_song: Option<String>,
        /// Whether the song should be shown on the item's tooltip.
        show_in_tooltip: bool,
    },
    /// marked as TODO on minecraft.wiki
    ProvidesBannerPatterns {
        /// A pattern identifier like `#minecraft:pattern_item/globe`.
        key: Ident<String>,
    },
    /// The recipes this knowledge book unlocks.
    Recipes {
        /// Always a Compound Tag.
        data: Compound,
    },
    /// The lodestone this compass points to.
    LodestoneTracker {
        /// Whether this lodestone points to a position, otherwise it spins
        /// randomly.
        has_global_position: bool,
        /// The dimension the compass points to. Only present if Has Global
        /// Position is true.
        dimension: Option<String>,
        /// The position the compass points to. Only present if Has Global
        /// Position is true.
        position: Option<(VarInt, VarInt, VarInt)>,
        /// Whether the component is removed when the associated lodestone is
        /// broken.
        tracked: bool,
    },
    /// Properties of a firework star.
    FireworkExplosion {
        /// See Firework Explosion.
        explosion: (VarInt, VarInt, Vec<VarInt>, VarInt, Vec<VarInt>, bool, bool),
    },
    /// Properties of a firework.
    Fireworks {
        /// The flight duration.
        flight_duration: VarInt,
        /// Number of elements in the following array.
        number_of_explosions: VarInt,
        /// The explosions.
        explosions: Vec<(VarInt, VarInt, Vec<VarInt>, VarInt, Vec<VarInt>, bool, bool)>,
    },
    /// Game Profile of a player's head.
    Profile {
        /// Whether the profile has a name.
        has_name: bool,
        /// The name of the profile. Only present if Has Name is true.
        name: Option<String>,
        /// Whether the profile has a unique ID.
        has_unique_id: bool,
        /// The unique ID of the profile. Only present if Has Unique ID is true.
        unique_id: Option<uuid::Uuid>,
        /// Number of elements in the following array.
        number_of_properties: VarInt,
        /// The properties.
        properties: Vec<(String, String, bool, Option<String>)>,
    },
    /// Sound played by a note block when this player's head is placed on top of
    /// it.
    NoteBlockSound {
        /// The sound.
        sound: String,
    },
    /// Patterns of a banner or banner applied to a shield.
    BannerPatterns {
        /// Number of elements in the following array.
        number_of_layers: VarInt,
        /// The layers.
        layers: Vec<(VarInt, Option<String>, Option<String>, VarInt)>,
    },
    /// Base color of the banner applied to a shield.
    BaseColor {
        /// The color.
        color: VarInt,
    },
    /// Decorations on the four sides of a pot.
    PotDecorations {
        /// The number of elements in the following array.
        number_of_decorations: VarInt,
        /// The decorations.
        decorations: Vec<VarInt>,
    },
    /// Items inside a container of any type.
    Container {
        /// The number of elements in the following array.
        number_of_items: VarInt,
        /// The items.
        // FIX: items: Vec<ItemStack>,
        items: Vec<u8>,
    },
    /// State of a block.
    BlockState {
        /// Number of elements in the following array.
        number_of_properties: VarInt,
        /// The properties.
        properties: Vec<(String, String)>,
    },
    /// Bees inside a hive.
    Bees {
        /// Number of elements in the following array.
        number_of_bees: VarInt,
        /// The bees.
        bees: Vec<(Compound, VarInt, VarInt)>,
    },
    /// Name of the necessary key to open this container.
    Lock {
        /// Always a String Tag.
        key: String,
    },
    /// Loot table for an unopened container.
    ContainerLoot {
        /// Always a Compound Tag.
        data: Compound,
    },
    /// Changes the sound that plays when the item breaks.
    BreakSound {
        sound_event: SoundId,
    },
    /// The biome variant of a villager.
    VillagerVariant {
        /// An ID in the minecraft:villager_type registry.
        variant: VarInt,
    },
    /// The variant of a wolf.
    WolfVariant {
        /// An ID in the minecraft:wolf_variant registry.
        variant: VarInt,
    },
    /// The type of sounds that a wolf makes.
    WolfSoundVariant {
        /// An ID in the minecraft:wolf_sound_variant registry.
        variant: VarInt,
    },
    /// The dye color of the wolf's collar.
    WolfCollar {
        color: VarInt, /* TODO: make this an enum https://minecraft.wiki/w/Java_Edition_protocol/Slot_data#Dye_Color */
    },
    /// The variant of a fox.
    FoxVariant {
        /// 0: red, 1: snow.
        variant: VarInt,
    },
    ///
    SalmonSize {
        /// 0: small, 1: medium, 2: large.
        size: VarInt,
    },
    /// The variant of a parrot.
    ParrotVariant {
        /// An ID in the minecraft:parrot_type registry.
        variant: VarInt,
    },
    /// The pattern of a tropical fish.
    TropicalFishPattern {
        /// 0: kob, 1: sunstreak, 2: snooper, 3: dasher, 4: brinely, 5: spotty,
        /// 6: flopper, 7: stripey, 8: glitter, 9: blockfish, 10: betty, 11:
        /// clayfish.
        pattern: VarInt, // TODO: maybe also enum?
    },
    /// The base color of a tropical fish.
    TropicalFishBaseColor {
        color: VarInt, /* TODO: make this an enum https://minecraft.wiki/w/Java_Edition_protocol/Slot_data#Dye_Color */
    },
    /// The pattern color of a tropical fish.
    TropicalFishPatternColor {
        color: VarInt, /* TODO: make this an enum https://minecraft.wiki/w/Java_Edition_protocol/Slot_data#Dye_Color */
    },
    /// The variant of a mooshroom.
    MooshroomVariant {
        /// 0: red, 1: brown.
        variant: VarInt,
    },
    /// The variant of a rabbit.
    RabbitVariant {
        // 0: brown, 1: white, 2: black, 3: white splotched, 4: gold, 5: salt, 6: evil.
        variant: VarInt, // TODO: enum?
    },
    /// An ID in the minecraft:pig_variant registry.
    PigVariant {
        /// An ID in the minecraft:pig_variant registry.
        variant: VarInt,
    },
    /// The variant of a cow.
    CowVariant {
        /// An ID in the minecraft:cow_variant registry.
        variant: VarInt,
    },
    /// The variant of a chicken.
    ChickenVariant {
        // TODO: implement
    },
    /// The variant of a frog.
    FrogVariant {
        /// An ID in the minecraft:frog_variant registry.
        variant: VarInt,
    },
    /// The variant of a horse.
    HorseVariant {
        /// 0: white, 1: creamy, 2: chestnut, 3: brown, 4: black, 5: gray, 6:
        /// dark brown.
        variant: VarInt, // TODO: enum?
    },
    /// The variant of a painting.
    PaintingVariant {
        // TODO: implement
    },
    /// The variant of a llama.
    LlamaVariant {
        /// 0: creamy, 1: white, 2: brown, 3: gray.
        variant: VarInt, // TODO: enum?
    },
    /// The variant of an axolotl.
    AxolotlVariant {
        /// 0: lucy, 1: wild, 2: gold, 3: cyan, 4: blue.
        variant: VarInt, // TODO: enum?
    },
    /// The variant of a cat.
    CatVariant {
        /// An ID in the minecraft:cat_variant registry.
        variant: VarInt, // TODO: enum?
    },
    /// The dye color of the cat's collar.
    CatCollar {
        color: VarInt, /* TODO: make this an enum https://minecraft.wiki/w/Java_Edition_protocol/Slot_data#Dye_Color */
    },
    /// The color of a sheep.
    SheepColor {
        color: VarInt, /* TODO: make this an enum https://minecraft.wiki/w/Java_Edition_protocol/Slot_data#Dye_Color */
    },
    /// The color of a shulker.
    ShulkerColor {
        color: VarInt, /* TODO: make this an enum https://minecraft.wiki/w/Java_Edition_protocol/Slot_data#Dye_Color */
    },
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

impl Rarity {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "common" => Some(Rarity::Common),
            "uncommon" => Some(Rarity::Uncommon),
            "rare" => Some(Rarity::Rare),
            "epic" => Some(Rarity::Epic),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct BlockPredicate {
    pub blocks: Option<IDSet>,
    pub properties: Option<Vec<Property>>,
    pub nbt: Option<Compound>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Property {
    pub name: String,
    pub is_exact_match: bool,
    pub exact_value: Option<String>,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
}

impl Encode for Property {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        self.name.encode(&mut w)?;
        self.is_exact_match.encode(&mut w)?;
        if let Some(ref exact_value) = self.exact_value {
            exact_value.encode(&mut w)?;
        }
        if let Some(ref min_value) = self.min_value {
            min_value.encode(&mut w)?;
        }
        if let Some(ref max_value) = self.max_value {
            max_value.encode(&mut w)?;
        }
        Ok(())
    }
}

impl<'a> Decode<'a> for Property {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        let name = String::decode(r)?;
        let is_exact_match = bool::decode(r)?;
        let exact_value = if is_exact_match {
            Some(String::decode(r)?)
        } else {
            None
        };
        let min_value = if !is_exact_match {
            Some(String::decode(r)?)
        } else {
            None
        };
        let max_value = if !is_exact_match {
            Some(String::decode(r)?)
        } else {
            None
        };
        Ok(Property {
            name,
            is_exact_match,
            exact_value,
            min_value,
            max_value,
        })
    }
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct ItemAttribute {
    pub effect: EntityAttribute,
    pub uuid: uuid::Uuid,
    pub name: String,
    pub value: f64,
    pub operation: EntityAttributeOperation,
    pub slot: AttributeSlot,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum AttributeSlot {
    Any,
    MainHand,
    OffHand,
    Hand,
    Feet,
    Legs,
    Chest,
    Head,
    Armor,
    Body,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum EquipSlot {
    Hand,
    Feet,
    Legs,
    Chest,
    Head,
    Offhand,
    Body,
}

#[derive(Clone, Copy, PartialEq, Debug, Encode, Decode)]
pub enum MapPostProcessingType {
    Lock,
    Expand,
}

#[derive(Clone, Copy, PartialEq, Debug, Encode, Decode)]
pub enum ConsumableAnimation {
    None,
    Eat,
    Drink,
    Block,
    Bow,
    Spear,
    Crossbow,
    Spyglass,
    TootHorn,
    Brush,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct ToolRule {
    pub blocks: IDSet,
    pub speed: Option<f32>,
    pub correct_drop_for_blocks: Option<bool>,
}

impl ItemComponent {
    pub fn id(&self) -> u32 {
        match self {
            ItemComponent::CustomData { .. } => 0,
            ItemComponent::MaxStackSize { .. } => 1,
            ItemComponent::MaxDamage { .. } => 2,
            ItemComponent::Damage { .. } => 3,
            ItemComponent::Unbreakable { .. } => 4,
            ItemComponent::CustomName { .. } => 5,
            ItemComponent::ItemName { .. } => 6,
            ItemComponent::ItemModel { .. } => 7,
            ItemComponent::Lore { .. } => 8,
            ItemComponent::Rarity { .. } => 9,
            ItemComponent::Enchantments { .. } => 10,
            ItemComponent::CanPlaceOn { .. } => 11,
            ItemComponent::CanBreak { .. } => 12,
            ItemComponent::AttributeModifiers { .. } => 13,
            ItemComponent::CustomModelData { .. } => 14,
            ItemComponent::TooltipDisplay { .. } => 15,
            ItemComponent::RepairCost { .. } => 16,
            ItemComponent::CreativeSlotLock { .. } => 17,
            ItemComponent::EnchantmentGlintOverride { .. } => 18,
            ItemComponent::IntangibleProjectile { .. } => 19,
            ItemComponent::Food { .. } => 20,
            ItemComponent::Consumable { .. } => 21,
            ItemComponent::UseRemainder { .. } => 22,
            ItemComponent::UseCooldown { .. } => 23,
            ItemComponent::DamageResistant { .. } => 24,
            ItemComponent::Tool { .. } => 25,
            ItemComponent::Weapon { .. } => 26,
            ItemComponent::Enchantable { .. } => 27,
            ItemComponent::Equippable { .. } => 28,
            ItemComponent::Repairable { .. } => 29,
            ItemComponent::Glider { .. } => 30,
            ItemComponent::TooltipStyle { .. } => 31,
            ItemComponent::DeathProtection { .. } => 32,
            ItemComponent::BlocksAttacks { .. } => 33,
            ItemComponent::StoredEnchantments { .. } => 34,
            ItemComponent::DyedColor { .. } => 35,
            ItemComponent::MapColor { .. } => 36,
            ItemComponent::MapId { .. } => 37,
            ItemComponent::MapDecorations { .. } => 38,
            ItemComponent::MapPostProcessing { .. } => 39,
            ItemComponent::ChargedProjectiles { .. } => 40,
            ItemComponent::BundleContents { .. } => 41,
            ItemComponent::PotionContents { .. } => 42,
            ItemComponent::PotionDurationScale { .. } => 43,
            ItemComponent::SuspiciousStewEffects { .. } => 44,
            ItemComponent::WritableBookContent { .. } => 45,
            ItemComponent::WrittenBookContent { .. } => 46,
            ItemComponent::Trim { .. } => 47,
            ItemComponent::DebugStickState { .. } => 48,
            ItemComponent::EntityData { .. } => 49,
            ItemComponent::BucketEntityData { .. } => 50,
            ItemComponent::BlockEntityData { .. } => 51,
            ItemComponent::Instrument { .. } => 52,
            ItemComponent::ProvidesTrimMaterial { .. } => 53,
            ItemComponent::OminousBottleAmplifier { .. } => 54,
            ItemComponent::JukeboxPlayable { .. } => 55,
            ItemComponent::ProvidesBannerPatterns { .. } => 56,
            ItemComponent::Recipes { .. } => 57,
            ItemComponent::LodestoneTracker { .. } => 58,
            ItemComponent::FireworkExplosion { .. } => 59,
            ItemComponent::Fireworks { .. } => 60,
            ItemComponent::Profile { .. } => 61,
            ItemComponent::NoteBlockSound { .. } => 62,
            ItemComponent::BannerPatterns { .. } => 63,
            ItemComponent::BaseColor { .. } => 64,
            ItemComponent::PotDecorations { .. } => 65,
            ItemComponent::Container { .. } => 66,
            ItemComponent::BlockState { .. } => 67,
            ItemComponent::Bees { .. } => 68,
            ItemComponent::Lock { .. } => 69,
            ItemComponent::ContainerLoot { .. } => 70,
            ItemComponent::BreakSound { .. } => 71,
            ItemComponent::VillagerVariant { .. } => 72,
            ItemComponent::WolfVariant { .. } => 73,
            ItemComponent::WolfSoundVariant { .. } => 74,
            ItemComponent::WolfCollar { .. } => 75,
            ItemComponent::FoxVariant { .. } => 76,
            ItemComponent::SalmonSize { .. } => 77,
            ItemComponent::ParrotVariant { .. } => 78,
            ItemComponent::TropicalFishPattern { .. } => 79,
            ItemComponent::TropicalFishBaseColor { .. } => 80,
            ItemComponent::TropicalFishPatternColor { .. } => 81,
            ItemComponent::MooshroomVariant { .. } => 82,
            ItemComponent::RabbitVariant { .. } => 83,
            ItemComponent::PigVariant { .. } => 84,
            ItemComponent::CowVariant { .. } => 85,
            ItemComponent::ChickenVariant { .. } => 86,
            ItemComponent::FrogVariant { .. } => 87,
            ItemComponent::HorseVariant { .. } => 88,
            ItemComponent::PaintingVariant { .. } => 89,
            ItemComponent::LlamaVariant { .. } => 90,
            ItemComponent::AxolotlVariant { .. } => 91,
            ItemComponent::CatVariant { .. } => 92,
            ItemComponent::CatCollar { .. } => 93,
            ItemComponent::SheepColor { .. } => 94,
            ItemComponent::ShulkerColor { .. } => 95,
        }
    }

    // Create a [`ItemComponent`] from a
    // [`valence_generated::item::SerItemComponent`] (which is generated by the
    // build script). fn from_serialized(serialized: SerItemComponent) -> Self {
    //     todo!()
    // }
}

impl ItemStack {
    pub const EMPTY: ItemStack = ItemStack {
        item: ItemKind::Air,
        count: 0,
        components: [const { None }; NUM_ITEM_COMPONENTS],
    };

    /// Creates a new item stack.
    ///
    /// If `components` is `None` then the default components of the
    /// [`ItemKind`] will be used (see [`ItemKind::components`]).]
    #[must_use]
    pub const fn new(item: ItemKind, count: i8) -> Self {
        Self {
            item,
            count,
            components: [const { None }; NUM_ITEM_COMPONENTS],
        }
    }

    /// Creates a new item stack with the vanilla default components for the
    /// given [`ItemKind`].
    pub fn new_vanilla(item: ItemKind, count: i8) -> Self {
        let components = item.default_components();

        Self {
            item,
            count,
            components,
        }
    }

    /// Read the components of the item stack.
    pub fn components(&self) -> Vec<&ItemComponent> {
        self.components
            .iter()
            .filter_map(|component| component.as_ref().map(|boxed| &**boxed))
            .collect()
    }

    /// Returns the default components for the [`ItemKind`].
    pub fn default_components(&self) -> Vec<ItemComponent> {
        self.item
            .default_components()
            .iter()
            .filter_map(|component| component.as_ref().map(|boxed| *boxed.clone()))
            .collect()
    }

    /// Attach a component to the item stack.
    pub fn insert_component(&mut self, component: ItemComponent) {
        let id = component.id() as usize;
        self.components[id] = Some(Box::new(component));
    }

    /// Remove a component from the item stack by its ID, see
    /// [`ItemComponent::id`].
    ///
    /// Returns the removed component if it was present, otherwise `None`.
    pub fn remove_component(&mut self, id: impl Into<usize>) -> Option<ItemComponent> {
        let id = id.into();
        if id < NUM_ITEM_COMPONENTS {
            self.components[id].take().map(|boxed| *boxed)
        } else {
            None
        }
    }

    /// Get a specific component by its ID, see [`ItemComponent::id`].
    pub fn get_component(&self, id: impl Into<usize>) -> Option<&ItemComponent> {
        let id = id.into();
        if id < NUM_ITEM_COMPONENTS {
            self.components[id].as_ref().map(|boxed| &**boxed)
        } else {
            None
        }
    }

    /// Get a mutable iterator over the components of the item stack.
    pub fn components_iter_mut(&mut self) -> impl Iterator<Item = &mut ItemComponent> {
        self.components
            .iter_mut()
            .filter_map(|component| component.as_mut().map(|boxed| &mut **boxed))
    }

    #[must_use]
    pub const fn with_count(mut self, count: i8) -> Self {
        self.count = count;
        self
    }

    #[must_use]
    pub const fn with_item(mut self, item: ItemKind) -> Self {
        self.item = item;
        self
    }

    #[must_use]
    pub fn with_components(mut self, components: Vec<ItemComponent>) -> Self {
        for component in components {
            self.insert_component(component);
        }
        self
    }

    pub const fn is_empty(&self) -> bool {
        matches!(self.item, ItemKind::Air) || self.count <= 0
    }
}

impl Encode for ItemStack {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        if self.is_empty() {
            VarInt(0).encode(&mut w)
        } else {
            VarInt(self.count as i32).encode(&mut w)?;
            self.item.encode(&mut w)?;

            let default_components = self.item.default_components();

            let (components_added, components_removed) = {
                let mut removed = Vec::new();
                let mut added = Vec::new();
                for i in 0..self.components.len() {
                    if self.components[i] != default_components[i] {
                        removed.push(i as u32);
                        if let Some(component) = &self.components[i] {
                            added.push(component);
                        }
                    }
                }

                (added, removed)
            };

            VarInt(components_added.len() as i32).encode(&mut w)?;
            VarInt(components_removed.len() as i32).encode(&mut w)?;

            for component in components_added {
                component.encode(&mut w)?;
            }

            for component in components_removed {
                component.encode(&mut w)?;
            }

            Ok(())
        }
    }
}

impl<'a> Decode<'a> for ItemStack {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        let count = VarInt::decode(r)?.0 as i8;
        if count == 0 {
            return Ok(ItemStack::EMPTY);
        };

        let item = ItemKind::decode(r)?;

        let default_components = item.default_components();

        let components_added_count = VarInt::decode(r)?.0 as usize;
        let components_removed_count = VarInt::decode(r)?.0 as usize;
        let mut components_added: Vec<ItemComponent> = Vec::with_capacity(components_added_count);
        let mut components_removed: Vec<u32> = Vec::with_capacity(components_removed_count);

        for _ in 0..components_added_count {
            let component = ItemComponent::decode(r)?;
            components_added.push(component);
        }

        for _ in 0..components_removed_count {
            let id = u32::decode(r)?;
            components_removed.push(id);
        }

        let mut components = default_components;

        for id in components_removed {
            components[id as usize] = None;
        }

        for component in components_added {
            let id = component.id() as usize;
            components[id] = Some(Box::new(component));
        }

        Ok(ItemStack {
            item,
            count,
            components,
        })
    }
}

pub trait ItemKindExt {
    /// Returns the default components for the [`ItemKind`].
    fn default_components(&self) -> [Option<Box<ItemComponent>>; NUM_ITEM_COMPONENTS];
}

impl ItemKindExt for ItemKind {
    fn default_components(&self) -> [Option<Box<ItemComponent>>; NUM_ITEM_COMPONENTS] {
        //     let ser_default_components = self.ser_components();
        //     let mut components = [const { None }; NUM_ITEM_COMPONENTS];

        //     for component in ser_default_components {
        //         let item_component = ItemComponent::from_serialized(component);
        //         let id = item_component.id() as usize;
        //         components[id] = Some(Box::new(item_component));
        //     }

        //     components
        // }

        [const { None }; NUM_ITEM_COMPONENTS]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_item_stack_is_empty() {
        let air_stack = ItemStack::new(ItemKind::Air, 10);
        let less_then_one_stack = ItemStack::new(ItemKind::Stone, 0);

        assert!(air_stack.is_empty());
        assert!(less_then_one_stack.is_empty());

        assert!(ItemStack::EMPTY.is_empty());

        let not_empty_stack = ItemStack::new(ItemKind::Stone, 10);

        assert!(!not_empty_stack.is_empty());
    }
}
