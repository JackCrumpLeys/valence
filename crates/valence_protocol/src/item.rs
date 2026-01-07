use std::fmt::Debug;
use std::io::Write;

use uuid::Uuid;
use valence_generated::attributes::EntityAttributeOperation;
pub use valence_generated::item::ItemKind;
use valence_generated::registry_id::RegistryId;
pub use valence_generated::sound::Sound;
use valence_ident::Ident;
use valence_nbt::Compound;
use valence_text::Text;

use crate::id_or::IdOr;
use crate::impls::cautious_capacity;
use crate::text_component::TextComponent;
use crate::{Decode, Encode, IDSet, VarInt};

const NUM_ITEM_COMPONENTS: usize = 96;
/// Controls the maximum recursion depth for encoding and decoding item
/// components.
const MAX_RECURSION_DEPTH: usize = 16;

#[derive(Clone, PartialEq, Debug, Copy)]
enum Patchable<T> {
    Default(T),
    /// `T`, `crc32c hash`
    Added((T, i32)),
    Removed,
    None,
}
impl<T> Patchable<T> {
    fn to_option(self) -> Option<T> {
        match self {
            Patchable::Default(t) => Some(t),
            Patchable::Added((t, _)) => Some(t),
            _ => None,
        }
    }

    fn to_option_ref(&self) -> Option<&T> {
        match self {
            Patchable::Default(t) => Some(t),
            Patchable::Added((t, _)) => Some(t),
            _ => None,
        }
    }
}

/// A stack of items in an inventory.
#[derive(Clone, PartialEq)]
pub struct ItemStack {
    pub item: ItemKind,
    pub count: i8,
    components: [Patchable<Box<ItemComponent>>; NUM_ITEM_COMPONENTS],
}

impl Default for ItemStack {
    fn default() -> Self {
        ItemStack::EMPTY
    }
}

impl Debug for ItemStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ItemStack")
            .field("item", &self.item)
            .field("count", &self.count)
            .field(
                "components",
                &self
                    .components
                    .iter()
                    .enumerate()
                    .filter_map(|(i, c)| c.to_option_ref().map(|comp| (i, comp)))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ItemComponent {
    /// Arbitrary NBT data that does not fit into other structured components.
    /// Used primarily by data-driven systems and server-side plugins to store
    /// state.
    CustomData(Compound),

    /// Overrides the default maximum stack size of the item.
    /// Allowed values are between 1 and 99.
    MaxStackSize(VarInt),

    /// The total durability of the item. This is the maximum value the 'Damage'
    /// component can reach before the item breaks.
    MaxDamage(VarInt),

    /// The current wear/tear of the item. 0 represents a new item,
    /// and higher values indicate more damage.
    Damage(VarInt),

    /// If present, the item will not take durability damage when used.
    /// Mechanical equivalent to the old 'Unbreakable: 1b' NBT tag.
    Unbreakable,

    /// A custom name for the item, typically set via an anvil.
    /// Usually rendered in italics by the client.
    CustomName(TextComponent),

    /// Overrides the base name of the item (e.g., "Stone").
    /// Unlike `CustomName`, this is not italicized by default.
    ItemName(TextComponent),

    /// References a specific model file in a resource pack.
    /// Allows a single Item ID to have multiple distinct visual appearances.
    ItemModel(String),

    /// Additional lines of text displayed below the item's name in the tooltip.
    Lore(Vec<TextComponent>),

    /// Determines the color of the item's name (Common/Uncommon/Rare/Epic).
    /// Also affects the default glint behavior in some contexts.
    Rarity(Rarity),

    /// A list of enchantments applied to the item and their corresponding
    /// levels.
    Enchantments(Vec<(RegistryId, VarInt)>),

    /// In Adventure mode, this restricts which blocks a player can place
    /// this specific block on.
    CanPlaceOn(Vec<BlockPredicate>),

    /// In Adventure mode, this restricts which blocks the player can break
    /// while holding this item.
    CanBreak(Vec<BlockPredicate>),

    /// Modifies the player's base attributes (like Attack Damage, Movement
    /// Speed, or Max Health) when this item is held or equipped.
    AttributeModifiers { modifiers: Vec<AttributeModifier> },

    /// Advanced visual overrides for resource packs.
    CustomModelData {
        /// Generic floating point values used by shaders or model predicates.
        floats: Vec<f32>,
        /// Boolean flags for toggling model parts.
        flags: Vec<bool>,
        /// String identifiers for selecting textures or sub-models.
        strings: Vec<String>,
        /// RGB integer colors for tinting specific model layers.
        colors: Vec<i32>,
    },

    /// Controls the visibility of the item's details.
    TooltipDisplay {
        /// If true, the entire tooltip (including name) is hidden.
        hide_tooltip: bool,
        /// A list of Component IDs that should not show their info in the
        /// tooltip.
        hidden_components: Vec<VarInt>,
    },

    /// The cumulative cost (in levels) added to anvil operations involving this
    /// item. Increases every time the item is repaired or modified.
    RepairCost(VarInt),

    /// Internal flag used for creative mode. If present, the item cannot be
    /// picked up or moved within specific creative tabs.
    CreativeSlotLock,

    /// Forces the "enchantment purple glow" to be either always on or always
    /// off, regardless of whether the item is actually enchanted.
    EnchantmentGlintOverride(bool),

    /// Used for projectiles (like arrows or tridents) to mark them as "ghost"
    /// items that cannot be picked back up by the player.
    IntangibleProjectile(Compound),

    /// Defines the nutritional value of the item when eaten.
    Food {
        /// How many hunger points (half-shanks) are restored.
        nutrition: VarInt,
        /// The multiplier applied to the nutrition to determine saturation.
        saturation_modifier: f32,
        /// If true, the player can eat this even if their hunger bar is full.
        can_always_eat: bool,
    },

    /// Defines how the item is used/consumed (e.g., eating, drinking, or using
    /// a bow).
    Consumable {
        /// The time in seconds required to finish using the item.
        consume_seconds: f32,
        /// The visual pose the player takes (Eat, Drink, Block, etc.).
        animation: ConsumableAnimation,
        /// The sound played during and after consumption.
        sound: IdOr<SoundEventDefinition>,
        /// Whether to spawn particle effects (like food crumbs) while using.
        has_consume_particles: bool,
        /// Status effects (like Poison or Speed) applied when consumption
        /// finishes.
        effects: Vec<ConsumeEffect>,
    },

    /// Defines an item that is returned to the inventory after this one is
    /// used. Example: Eating Mushroom Stew returns an empty Bowl.
    UseRemainder(Box<ItemStack>),

    /// Prevents the item from being used again for a set duration.
    UseCooldown {
        /// Duration of the cooldown in seconds.
        seconds: f32,
        /// Optional group ID. All items with the same group will share the
        /// cooldown.
        cooldown_group: Option<String>,
    },

    /// Prevents the item from being destroyed by certain damage types (e.g.,
    /// fire-resistant Netherite).
    DamageResistant(String),

    /// Configures how this item mines blocks.
    Tool {
        /// Specific rules for block sets (e.g., "Pickaxes mine stones fast").
        rules: Vec<ToolRule>,
        /// The mining speed used if no specific rule matches.
        default_mining_speed: f32,
        /// Durability lost per block broken.
        damage_per_block: VarInt,
        /// If false, this tool cannot break blocks in Creative mode.
        can_destroy_blocks_in_creative: bool,
    },

    /// Statistics for attacking.
    Weapon {
        /// Base damage added to the player's attack.
        damage_per_attack: VarInt,
        /// The duration (in seconds) that blocking is disabled after an attack
        /// is landed.
        disable_blocking_for_seconds: f32,
    },

    /// Determines how many experience points the item "absorbs" in an
    /// enchanting table.
    Enchantable(VarInt),

    /// Logic for equipping the item.
    Equippable {
        /// Which body slot this item fits into (Head, Chest, etc.).
        slot: EquipSlot,
        /// Sound played when the item is equipped.
        equip_sound: IdOr<SoundEventDefinition>,
        /// Reference to an equipment-specific model (like 3D armor).
        model: Option<String>,
        /// Texture used when the player's camera is "inside" the item (like a
        /// Pumpkin).
        camera_overlay: Option<String>,
        /// Which entity types are allowed to wear this item.
        allowed_entities: Option<IDSet>,
        /// Whether a Dispenser can equip this onto an entity.
        dispensable: bool,
        /// Whether right-clicking allows swapping this with currently equipped
        /// armor.
        swappable: bool,
        /// If true, the item takes durability damage when the wearer is hurt.
        damage_on_hurt: bool,
    },

    /// Items that can be used in an anvil to repair this item.
    Repairable(IDSet),

    /// Enables Elytra-style flight physics when equipped.
    Glider,

    /// References a custom sprite used as the background of the item's tooltip.
    TooltipStyle(String),

    /// Replicates the "Totem of Undying" behavior.
    DeathProtection(Vec<ConsumeEffect>),

    /// Shield-specific combat logic.
    BlocksAttacks {
        /// Delay in seconds before blocking becomes active.
        block_delay_seconds: f32,
        /// Scale factor for cooldowns when blocking.
        disable_cooldown_scale: f32,
        /// How much damage is absorbed from specific sources.
        damage_reductions: Vec<DamageReduction>,
        /// Minimum damage required for the shield to take durability loss.
        item_damage_threshold: f32,
        /// Flat durability loss per block.
        item_damage_base: f32,
        /// Multiplier for durability loss based on damage blocked.
        item_damage_factor: f32,
        /// Damage type tag that pierces this shield's blocking logic.
        bypassed_by: Option<String>,
        /// Sound played when a hit is successfully blocked.
        block_sound: Option<IdOr<SoundEventDefinition>>,
        /// Sound played when the shield is disabled (e.g., by an Axe).
        disable_sound: Option<IdOr<SoundEventDefinition>>,
    },

    /// Enchantments contained within an Enchanted Book.
    StoredEnchantments {
        enchantments: Vec<(RegistryId, VarInt)>,
        show_in_tooltip: bool,
    },

    /// RGB color for leather armor or other dyeable items.
    DyedColor {
        /// The packed RGB integer.
        color: i32,
        /// Whether the "Dyed" text is visible in the tooltip.
        show_in_tooltip: bool,
    },

    /// The color used for markings on a Map item.
    MapColor(i32),

    /// The numerical ID associated with a filled Map.
    MapId(VarInt),

    /// NBT data defining markers, banners, and icons shown on a map.
    MapDecorations(Compound),

    /// Tracking state for map expansion or locking.
    MapPostProcessing(MapPostProcessingType),

    /// Items currently loaded into a Crossbow.
    ChargedProjectiles(Vec<ItemStack>),

    /// Items stored inside a Bundle.
    BundleContents(Vec<ItemStack>),

    /// Data for Potion items, including their base type and custom effects.
    PotionContents {
        /// The base potion type (e.g., "Invisibility").
        potion_id: Option<RegistryId>,
        /// Custom color for the liquid, overrides the potion's default.
        custom_color: Option<i32>,
        /// Additional status effects not included in the base potion type.
        custom_effects: Vec<PotionEffect>,
        /// An optional name for the specific potion mixture.
        custom_name: Option<String>,
    },

    /// Multiplier for the duration of effects applied by this potion.
    PotionDurationScale(f32),

    /// Effects granted by eating Suspicious Stew.
    SuspiciousStewEffects(Vec<(RegistryId, VarInt)>),

    /// Pages and filtering information for a Book and Quill.
    WritableBookContent { pages: Vec<WritablePage> },

    /// Finalized content for a Written Book.
    WrittenBookContent {
        /// The displayed title.
        raw_title: String,
        /// The title after passing through server-side chat filters.
        filtered_title: Option<String>,
        /// The username of the player who signed the book.
        author: String,
        /// How many times the book has been copied (Original, Copy of Original,
        /// etc.).
        generation: VarInt,
        /// Page contents (Rich Text).
        pages: Vec<WrittenPage>,
        /// Whether entity selectors (like @p) have been resolved.
        resolved: bool,
    },
    /// Visual armor customization (Pattern and Material).
    Trim {
        material: IdOr<TrimMaterial>,
        pattern: IdOr<TrimPattern>,
        /// Whether the "Armor Trim" lines show in the tooltip.
        show_in_tooltip: bool,
    },

    /// Internal state for the Debug Stick, tracking property toggles.
    DebugStickState(Compound),

    /// NBT data used to modify an entity when it is spawned from an item (Spawn
    /// Eggs).
    EntityData { id: RegistryId, data: Compound },

    /// NBT data for entities inside a Bucket (like Fish or Axolotls).
    BucketEntityData(Compound),

    /// NBT data for the Block Entity created when this item is placed (Chests,
    /// Signs).
    BlockEntityData { id: RegistryId, data: Compound },

    /// The specific sound and duration associated with a Goat Horn.
    Instrument(IdOr<InstrumentDefinition>),

    /// Marks an item as a valid material for the Armor Trim system.
    ProvidesTrimMaterial(ModePair<String, IdOr<TrimMaterial>>),

    /// The level of Bad Omen granted by an Ominous Bottle (0-4).
    OminousBottleAmplifier(VarInt),

    /// Configuration for items that can be played in a Jukebox.
    JukeboxPlayable {
        /// Reference to a Jukebox Song.
        song: ModePair<String, IdOr<JukeboxSong>>,
        show_in_tooltip: bool,
    },

    /// Marks an item as a valid pattern for the Loom (Banner Patterns).
    ProvidesBannerPatterns(String),

    /// A list of recipe IDs that a Knowledge Book will teach the player.
    Recipes(Compound),

    /// Tracking data for a Compass pointing to a specific Lodestone.
    LodestoneTracker {
        /// The dimension and coordinate of the target. None if the compass is
        /// spinning.
        target: Option<LodestoneTarget>,
        /// If true, the compass becomes a normal compass if the lodestone is
        /// destroyed.
        tracked: bool,
    },

    /// Individual explosion properties for a Firework Star.
    FireworkExplosion(FireworkExplosionData),

    /// Flight and explosion data for a Firework Rocket.
    Fireworks {
        flight_duration: VarInt,
        explosions: Vec<FireworkExplosionData>,
    },

    /// Data for a Player Head, including the skin texture and UUID.
    Profile(ResolvableProfile),

    /// The sound played by a Note Block if this player head is placed on top of
    /// it.
    NoteBlockSound(String),

    /// Visual layers for a Banner or Shield.
    BannerPatterns(Vec<BannerLayer>),

    /// The base dye color for a Banner.
    BaseColor(VarInt),

    /// The four item IDs used as patterns on a Decorated Pot.
    PotDecorations(Vec<RegistryId>),

    /// The inventory contents of a block (like a Chest or Shulker Box).
    Container(Vec<ItemStack>),

    /// Block state property overrides (e.g., "lit: true").
    BlockState(Vec<(String, String)>),

    /// Data for bees currently inside a Beehive item.
    Bees(Vec<BeeData>),

    /// The "Key" name required to open a container if it has a Lock component.
    Lock(String),

    /// Reference to a Loot Table for an unopened chest.
    ContainerLoot(Compound),

    /// Overrides the default sound played when this specific item breaks.
    BreakSound(IdOr<SoundEventDefinition>),

    /// Biome-specific variant of a Villager (e.g., Desert, Plains).
    VillagerVariant(RegistryId),

    /// Skin variant for a Wolf.
    WolfVariant(RegistryId),

    /// Determines the bark/growl sounds for a Wolf.
    WolfSoundVariant(RegistryId),

    /// Dye color of a Wolf's collar.
    WolfCollar(DyeColor),

    /// Type of Fox (Red or Snow).
    FoxVariant(FoxType),

    /// Size of a Salmon (Small, Medium, Large).
    SalmonSize(SalmonScale),

    /// Color of a Parrot.
    ParrotVariant(ParrotType),

    /// Pattern type for a Tropical Fish.
    TropicalFishPattern(TropicalFishPattern),

    /// Primary color of a Tropical Fish.
    TropicalFishBaseColor(DyeColor),

    /// Secondary color of a Tropical Fish.
    TropicalFishPatternColor(DyeColor),

    /// Type of Mooshroom (Red or Brown).
    MooshroomVariant(MooshroomType),

    /// Breed of a Rabbit.
    RabbitVariant(RabbitType),

    /// Skin variant for a Pig.
    PigVariant(RegistryId),

    /// Skin variant for a Cow.
    CowVariant(RegistryId),

    /// Skin variant for a Chicken.
    ChickenVariant(ModePair<String, RegistryId>),

    /// Biome variant for a Frog.
    FrogVariant(RegistryId),

    /// Color and marking variant for a Horse.
    HorseVariant(HorseColor),

    /// The specific painting texture and dimensions.
    PaintingVariant(IdOr<PaintingVariantDefinition>),

    /// Color variant for a Llama.
    LlamaVariant(LlamaColor),

    /// Color variant for an Axolotl.
    AxolotlVariant(AxolotlType),

    /// Breed variant for a Cat.
    CatVariant(RegistryId),

    /// Dye color of a Cat's collar.
    CatCollar(DyeColor),

    /// Natural wool color of a Sheep.
    SheepColor(DyeColor),

    /// Shell color of a Shulker.
    ShulkerColor(DyeColor),
}

/// A helper struct for protcol fields that start with a "Mode" byte.
///
/// In 1.21, several components (like Jukebox Songs or Trim Materials) are
/// encoded as:
/// - Byte `0`: Followed by Type A (usually a String Identifier).
/// - Byte `1`: Followed by Type B (usually an ID or Inline Definition).
#[derive(Clone, PartialEq, Debug)]
pub enum ModePair<A, B> {
    /// Mode 0: Usually references a registry key by name.
    Mode0(A),
    /// Mode 1: Usually references a registry ID or defines the data inline.
    Mode1(B),
}

impl<A: Encode, B: Encode> Encode for ModePair<A, B> {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        match self {
            ModePair::Mode0(a) => {
                0_u8.encode(&mut w)?;
                a.encode(w)
            }
            ModePair::Mode1(b) => {
                1_u8.encode(&mut w)?;
                b.encode(w)
            }
        }
    }
}
impl<'a, A: Decode<'a>, B: Decode<'a>> Decode<'a> for ModePair<A, B> {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        let mode = u8::decode(r)?;
        match mode {
            0 => Ok(ModePair::Mode0(A::decode(r)?)),
            1 => Ok(ModePair::Mode1(B::decode(r)?)),
            _ => anyhow::bail!("Invalid ModePair byte: {mode}"),
        }
    }
}

/// Defines a rule for matching a block in the world.
/// Used by `CanPlaceOn` and `CanBreak` in Adventure Mode.
#[derive(Clone, PartialEq, Debug, Encode)]
pub struct BlockPredicate {
    /// If None, matches any block ID.
    pub blocks: Option<IDSet>,

    /// Matches specific block state properties (e.g., `lit=true`).
    pub properties: Option<Vec<Property>>,

    /// Matches the Block Entity's NBT data.
    pub nbt: Option<Compound>,

    /// (1.21+) Matches if the block drops an item containing these EXACT
    /// components. This is a strict equality check.
    pub exact_components: Vec<ExactComponentMatcher>,

    /// (1.21+) Matches if the block drops an item containing specific NBT
    /// structures within specific components.
    pub partial_components: Vec<PartialComponentMatcher>,
}

// A specific Block State property requirement.
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct Property {
    /// The name of the property (e.g., "facing", "waterlogged").
    pub name: String,

    /// Either an exact value or a min/max range for the property.
    pub value: PropertyValue,
}

#[derive(Clone, PartialEq, Debug)]
pub enum PropertyValue {
    /// An exact string value.
    Exact(String),
    /// A min max string value.
    MinMax { min: String, max: String },
}

// encoded as bool followed by one if true or min and max if false
impl Encode for PropertyValue {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        match self {
            PropertyValue::Exact(v) => {
                true.encode(&mut w)?;
                v.encode(w)
            }
            PropertyValue::MinMax { min, max } => {
                false.encode(&mut w)?;
                min.encode(&mut w)?;
                max.encode(w)
            }
        }
    }
}
impl<'a> Decode<'a> for PropertyValue {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        let is_exact = bool::decode(r)?;
        if is_exact {
            let v = String::decode(r)?;
            Ok(PropertyValue::Exact(v))
        } else {
            let min = String::decode(r)?;
            let max = String::decode(r)?;
            Ok(PropertyValue::MinMax { min, max })
        }
    }
}

/// Matches a component exactly.
#[derive(Clone, PartialEq, Debug, Encode)]
pub struct ExactComponentMatcher {
    /// The ID of the component to check.
    pub component_type: VarInt,
    /// The encoded data of that component.
    pub component_data: ItemComponent,
}

/// Matches a subset of data within a component using NBT.
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct PartialComponentMatcher {
    /// The ID of the component to check.
    pub component_type: VarInt,
    /// An NBT matcher to apply to that component's internal data.
    pub predicate: Compound,
}

/// Modifies a player's attributes (like Strength or Speed).
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct AttributeModifier {
    /// The ID of the attribute to modify in the registry.
    pub attribute_id: RegistryId,

    /// A unique identifier for this modifier instance.
    /// Used to prevent stacking the same modifier multiple times from different
    /// sources.
    pub modifier_id: Ident<String>,

    /// The numerical amount to change the attribute by.
    pub value: f64,

    /// How the math is applied.
    /// (Add): X = X + Value
    /// (Multiply Base): X = X + (`BaseValue` * Value)
    /// (Multiply Total): X = X * (1 + Value)
    pub operation: EntityAttributeOperation,

    /// Which slot the item must be in for this to work.
    pub slot: AttributeSlot,
}

/// Defines custom mining speed logic for a tool.
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct ToolRule {
    /// The blocks this rule applies to.
    pub blocks: IDSet,

    /// If present, overrides the mining speed for these blocks.
    pub speed: Option<f32>,

    /// If present and true, this tool is considered "correct" for the block
    /// (meaning the block will drop items when broken).
    pub correct_drop_for_blocks: Option<bool>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct LodestoneTarget {
    /// The namespaced key of the dimension (e.g., "`minecraft:the_nether`").
    pub dimension: String,

    /// The precise X, Y, Z coordinates of the Lodestone block.
    pub position: (VarInt, VarInt, VarInt),
}

/// Defines a sound event, either by referencing the registry or defining it
/// on the fly.
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct SoundEventDefinition {
    /// The identifier of the sound (e.g., "minecraft:entity.pig.ambient").
    /// In 1.21, this can be a direct String or a Registry ID.
    pub sound: ModePair<String, RegistryId>,

    /// A fixed range (in blocks) for the sound. If None, uses the default.
    pub range: Option<f32>,
}

/// Defines a material used to trim armor (e.g., Gold, Amethyst).
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct TrimMaterial {
    /// Corresponds to "Suffix" in the Wiki.
    /// This string is appended to the texture path (e.g., "amethyst" ->
    /// "`trims/items/leggings_trim_amethyst`").
    pub asset_name: String,

    /// Allows specific armor materials to use a different texture suffix.
    ///
    /// Structure:
    /// - Key: Armor Material Model Name (Identifier, e.g.,
    ///   "minecraft:netherite")
    /// - Value: Overridden Asset Name (String, e.g., "`amethyst_darker`")
    pub overrides: Vec<(Ident<String>, String)>,

    /// Corresponds to "Description" in the Wiki.
    /// The text displayed in the item tooltip (e.g., "Amethyst Material").
    pub description: TextComponent,
}

/// Defines the shape/pattern of the armor trim (e.g., Vex, Coast).
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct TrimPattern {
    /// The asset ID for the texture pattern.
    pub asset_id: String,

    /// The Smithing Template item required to apply this pattern.
    pub template_item: RegistryId,

    /// The text displayed in the tooltip (e.g., "Vex Armor Trim").
    pub description: TextComponent,

    /// If true, the pattern is applied as a "Decal" (no color blending).
    pub decal: bool,
}

/// Defines a Goat Horn instrument.
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct InstrumentDefinition {
    /// The sound played when the horn is used.
    pub sound_event: IdOr<SoundEventDefinition>,

    /// How long the horn plays (in seconds).
    pub use_duration: f32,

    /// The audible range of the horn.
    pub range: f32,

    /// The description shown in the tooltip (e.g., "Ponder").
    pub description: TextComponent,
}

/// Defines a Music Disc song.
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct JukeboxSong {
    /// The sound event to play.
    pub sound_event: IdOr<SoundEventDefinition>,

    /// The song title shown in the "Now Playing" action bar.
    pub description: Text,

    /// The duration of the song in seconds.
    pub length_seconds: f32,

    /// The Redstone signal strength (0-15) emitted by the Jukebox while
    /// playing.
    pub comparator_output: VarInt,
}

/// Defines a variant of a painting entity.
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct PaintingVariantDefinition {
    /// The path to the texture in the resource pack.
    pub asset_id: String,

    /// Width in blocks (1 block = 16 pixels).
    pub width: VarInt,

    /// Height in blocks.
    pub height: VarInt,
}

/// Defines a single explosion in a Firework Rocket.
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct FireworkExplosionData {
    /// The shape (Small Ball, Large Ball, Star, Creeper, Burst).
    pub shape: VarInt,

    /// List of RGB integers for the initial explosion colors.
    pub colors: Vec<i32>,

    /// List of RGB integers that the particles fade into.
    pub fade_colors: Vec<i32>,

    /// If true, particles leave a trail behind them.
    pub has_trail: bool,

    /// If true, particles crackle/flash after the explosion.
    pub has_twinkle: bool,
}

/// Defines a layer on a Banner.
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct BannerLayer {
    /// The pattern type (Flower, Skull, Stripe, etc.).
    pub pattern: IdOr<BannerPattern>,

    /// The dye color ID (0-15) for this layer.
    pub color: DyeColor,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct BannerPattern {
    /// The texture identifier (e.g., "minecraft:flower").
    pub asset_id: String,

    /// The translation key for the pattern name (e.g.,
    /// "block.minecraft.banner.flower").
    pub translation_key: String,
}

/// A page in a Book and Quill (Writable).
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct WritablePage {
    /// The raw text entered by the player.
    pub raw: String,

    /// If the server runs a chat filter, this is the filtered version.
    /// If None, the raw text is considered safe to display.
    pub filtered: Option<String>,
}

/// A page in a Finished Book (Written).
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct WrittenPage {
    /// The JSON text component for the page content.
    pub raw: TextComponent,

    /// Optional filtered version for chat safety settings.
    pub filtered: Option<TextComponent>,
}

/// Represents a Player's Game Profile (Skin/UUID).
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct ResolvableProfile {
    /// The player's username.
    pub name: Option<String>,

    /// The player's UUID.
    pub id: Option<Uuid>,

    /// Properties, primarily the "textures" property containing the skin URL.
    pub properties: Vec<ProfileProperty>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct ProfileProperty {
    pub name: String,
    /// The base64 encoded value.
    pub value: String,
    /// The optional public key signature (Yggdrasil).
    pub signature: Option<String>,
}

/// Information about a Bee inside a Beehive.
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct BeeData {
    /// The NBT data of the Bee entity itself (Health, Name, etc.).
    pub entity_data: Compound,

    /// How many ticks the bee has been inside the hive.
    pub ticks_in_hive: VarInt,

    /// The minimum ticks required before the bee can leave.
    pub min_ticks_in_hive: VarInt,
}

/// A wrapper for the various effects caused by consuming an item.
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct ConsumeEffect {
    /// The registry ID of the effect type (`ApplyEffects`, Teleport, etc.).
    pub type_id: VarInt,

    /// The effect data. Note: The protocol doesn't wrap this in a neat enum,
    /// it sends the data immediately after the ID.
    /// You must ensure your Decode logic matches the `type_id` to the correct
    /// variant here.
    pub data: ConsumeEffectData,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum ConsumeEffectData {
    /// Type 0: Apply Effects
    ApplyEffects {
        /// List of potion effects to apply.
        effects: Vec<PotionEffect>,
        /// Chance (0.0 to 1.0) that these effects are applied.
        probability: f32,
    },
    /// Type 1: Remove Effects
    RemoveEffects(IDSet), // Set of Potion IDs to cure/remove.
    /// Type 2: Clear All Effects
    ClearAllEffects,
    /// Type 3: Teleport Randomly (Chorus Fruit behavior)
    TeleportRandomly {
        /// The horizontal radius to search for a safe spot.
        diameter: f32,
    },
    /// Type 4: Play Sound
    PlaySound(IdOr<SoundEventDefinition>),
}

/// A standard Potion Effect.
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct PotionEffect {
    /// The ID of the effect (Speed, Jump Boost, etc.).
    pub id: RegistryId,

    /// The level of the effect (0 = Level 1, 1 = Level 2).
    pub amplifier: VarInt,

    /// Duration in ticks. -1 indicates infinite duration.
    pub duration: VarInt,

    /// If true, particles are translucent (like Beacon effects).
    pub ambient: bool,

    /// If false, no particles are shown.
    pub show_particles: bool,

    /// If true, the effect icon is displayed in the top-right of theinventory.
    pub show_icon: bool,
}

/// Shield logic for reducing damage.
#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct DamageReduction {
    /// The angle (in degrees) in front of the player that is blocked.
    pub horizontal_blocking_angle: f32,

    /// Specific damage types this reduction applies to. None = All.
    pub damage_type: Option<IDSet>,

    /// Flat amount of damage removed.
    pub base: f32,

    /// Percentage of remaining damage removed (0.0 to 1.0).
    pub factor: f32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum Rarity {
    Common,   // White
    Uncommon, // Yellow
    Rare,     // Aqua
    Epic,     // Purple
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum MapPostProcessingType {
    Lock,  // The map has been locked in a Cartography Table.
    Scale, // The map is being zoomed out.
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum ConsumableAnimation {
    None,
    Eat,
    Drink,
    Block, // Shield block animation
    Bow,
    Spear, // Trident throw
    Crossbow,
    Spyglass,
    TootHorn,
    Brush,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum EquipSlot {
    MainHand,
    Feet,
    Legs,
    Chest,
    Head,
    OffHand,
    Body, // Horse armor / Llama carpet
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub enum AttributeSlot {
    Any,
    MainHand,
    OffHand,
    Hand, // MainHand or OffHand
    Feet,
    Legs,
    Chest,
    Head,
    Armor, // Any armor slot
    Body,
}

#[derive(Clone, Copy, PartialEq, Debug, Encode, Decode)]
pub enum DyeColor {
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}

#[derive(Clone, Copy, PartialEq, Debug, Encode, Decode)]
pub enum FoxType {
    Red,
    Snow,
}

#[derive(Clone, Copy, PartialEq, Debug, Encode, Decode)]
pub enum SalmonScale {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Copy, PartialEq, Debug, Encode, Decode)]
pub enum ParrotType {
    RedBlue,
    Blue,
    Green,
    YellowBlue,
    Gray,
}

#[derive(Clone, Copy, PartialEq, Debug, Encode, Decode)]
pub enum TropicalFishPattern {
    Kob,
    Sunstreak,
    Snooper,
    Dasher,
    Brinely,
    Spotty,
    Flopper,
    Stripey,
    Glitter,
    Blockfish,
    Betty,
    Clayfish,
}

#[derive(Clone, Copy, PartialEq, Debug, Encode, Decode)]
pub enum MooshroomType {
    Red,
    Brown,
}

#[derive(Clone, Copy, PartialEq, Debug, Encode, Decode)]
pub enum RabbitType {
    Brown,
    White,
    Black,
    WhiteSplotched,
    Gold,
    Salt,
    Evil, // "Toast"
}

#[derive(Clone, Copy, PartialEq, Debug, Encode, Decode)]
pub enum HorseColor {
    White,
    Creamy,
    Chestnut,
    Brown,
    Black,
    Gray,
    DarkBrown,
}

#[derive(Clone, Copy, PartialEq, Debug, Encode, Decode)]
pub enum LlamaColor {
    Creamy,
    White,
    Brown,
    Gray,
}

#[derive(Clone, Copy, PartialEq, Debug, Encode, Decode)]
pub enum AxolotlType {
    Lucy, // Pink
    Wild, // Brown
    Gold,
    Cyan,
    Blue,
}

#[derive(Clone, PartialEq, Debug)]
pub struct HashedItemStack {
    pub item: ItemKind,
    pub count: i8,
    components: [Patchable<()>; NUM_ITEM_COMPONENTS],
}
impl HashedItemStack {
    pub const EMPTY: Self = Self {
        item: ItemKind::Air,
        count: 0,
        components: [const { Patchable::None }; NUM_ITEM_COMPONENTS],
    };

    pub const fn is_empty(&self) -> bool {
        matches!(self.item, ItemKind::Air) || self.count <= 0
    }
}

impl From<ItemStack> for HashedItemStack {
    fn from(stack: ItemStack) -> Self {
        Self {
            item: stack.item,
            count: stack.count,
            components: stack.components.map(|c| match c {
                Patchable::Default(_) => Patchable::Default(()),
                Patchable::Added((_, h)) => Patchable::Added(((), h)),
                Patchable::Removed => Patchable::Removed,
                Patchable::None => Patchable::None,
            }),
        }
    }
}

impl ItemComponent {
    pub fn id(&self) -> u32 {
        match self {
            ItemComponent::CustomData { .. } => 0,
            ItemComponent::MaxStackSize { .. } => 1,
            ItemComponent::MaxDamage { .. } => 2,
            ItemComponent::Damage { .. } => 3,
            ItemComponent::Unbreakable => 4,
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
            ItemComponent::CreativeSlotLock => 17,
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
            ItemComponent::Glider => 30,
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

    pub fn hash(&self) -> i32 {
        0
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
        components: [const { Patchable::None }; NUM_ITEM_COMPONENTS],
    };

    /// Creates a new item stack without any components.
    #[must_use]
    pub const fn new(item: ItemKind, count: i8) -> Self {
        Self {
            item,
            count,
            components: [const { Patchable::None }; NUM_ITEM_COMPONENTS],
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
            .filter_map(|component| component.to_option_ref())
            .map(|boxed| &**boxed)
            .collect()
    }

    /// Returns the default components for the [`ItemKind`].
    pub fn default_components(&self) -> Vec<ItemComponent> {
        self.item
            .default_components()
            .iter()
            .filter_map(|component| component.to_option_ref().map(|b| &**b))
            .cloned()
            .collect()
    }

    /// Attach a component to the item stack.
    pub fn insert_component(&mut self, component: ItemComponent) {
        let id = component.id() as usize;
        if let Patchable::Default(default) = &self.components[id] {
            // We don't need to add a components if its default for the item kind.
            if **default == component {
                return;
            }
        }

        let hash = component.hash();
        self.components[id] = Patchable::Added((Box::new(component), hash));
    }

    /// Remove a component from the item stack by its ID, see
    /// [`ItemComponent::id`].String
    ///
    /// Returns the removed component if it was present, otherwise `None`.
    pub fn remove_component<I: Into<usize>>(&mut self, id: I) -> Option<ItemComponent> {
        let id = id.into();
        if id < NUM_ITEM_COMPONENTS {
            std::mem::replace(&mut self.components[id], Patchable::Removed)
                .to_option()
                .map(|boxed| *boxed)
        } else {
            None
        }
    }

    /// Get a specific component by its ID, see [`ItemComponent::id`].
    pub fn get_component<I: Into<usize>>(&self, id: I) -> Option<&ItemComponent> {
        let id = id.into();
        if id < NUM_ITEM_COMPONENTS {
            match &self.components[id] {
                Patchable::Added((component, _)) | Patchable::Default(component) => {
                    Some(&**component)
                }
                _ => None,
            }
        } else {
            None
        }
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

    pub(crate) fn encode_recursive(
        &self,
        mut w: impl Write,
        prefixed: bool,
    ) -> Result<(), anyhow::Error> {
        if self.is_empty() {
            VarInt(0).encode(w)
        } else {
            // Break recursion loop by erasing the type
            let w: &mut dyn Write = &mut w;

            VarInt(i32::from(self.count)).encode(&mut *w)?;
            self.item.encode(&mut *w)?;

            let mut added = Vec::new();
            let mut removed = Vec::new();

            for (i, patch) in self.components.iter().enumerate() {
                match patch {
                    Patchable::Added((comp, _)) => added.push((i, comp)),
                    Patchable::Removed => removed.push(i),
                    _ => {}
                }
            }

            // Encode Added & removed
            VarInt(added.len() as i32).encode(&mut *w)?;
            VarInt(removed.len() as i32).encode(&mut *w)?;

            for (id, comp) in added {
                VarInt(id as i32).encode(&mut *w)?;
                if prefixed {
                    // We need to record the length of the component data.
                    // Then we encode len then the data.
                    //
                    // We use a dummy writer to avoid allocator pressue at the cost of cpu.

                    struct ByteCounter {
                        count: usize,
                    }

                    impl Write for ByteCounter {
                        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                            self.count += buf.len();
                            Ok(buf.len())
                        }

                        fn flush(&mut self) -> std::io::Result<()> {
                            Ok(())
                        }
                    }

                    // Encode to the counter to determine the length
                    let mut counter = ByteCounter { count: 0 };
                    comp.encode(&mut counter)?;

                    // Write the length prefix
                    VarInt(counter.count as i32).encode(&mut *w)?;

                    // Real run: Encode the data to the actual writer
                    comp.encode(&mut *w)?;
                } else {
                    comp.encode(&mut *w)?;
                }
            }

            for id in removed {
                VarInt(id as i32).encode(&mut *w)?;
            }

            Ok(())
        }
    }
}

impl Encode for ItemStack {
    fn encode(&self, w: impl Write) -> anyhow::Result<()> {
        self.encode_recursive(w, false)
    }
}

impl Encode for ItemComponent {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        // Break recursion loop by erasing the type
        let w: &mut dyn Write = &mut w;

        match self {
            ItemComponent::CustomData(v) => v.encode(w),
            ItemComponent::MaxStackSize(v) => v.encode(w),
            ItemComponent::MaxDamage(v) => v.encode(w),
            ItemComponent::Damage(v) => v.encode(w),
            ItemComponent::Unbreakable => Ok(()),
            ItemComponent::CustomName(v) => v.encode(w),
            ItemComponent::ItemName(v) => v.encode(w),
            ItemComponent::ItemModel(v) => v.encode(w),
            ItemComponent::Lore(v) => v.encode(w),
            ItemComponent::Rarity(v) => v.encode(w),
            ItemComponent::Enchantments(v) => v.encode(w),
            ItemComponent::CanPlaceOn(v) => v.encode(w),
            ItemComponent::CanBreak(v) => v.encode(w),
            ItemComponent::AttributeModifiers { modifiers } => modifiers.encode(w),
            ItemComponent::CustomModelData {
                floats,
                flags,
                strings,
                colors,
            } => {
                floats.encode(&mut *w)?;
                flags.encode(&mut *w)?;
                strings.encode(&mut *w)?;
                colors.encode(w)
            }
            ItemComponent::TooltipDisplay {
                hide_tooltip,
                hidden_components,
            } => {
                hide_tooltip.encode(&mut *w)?;
                hidden_components.encode(w)
            }
            ItemComponent::RepairCost(v) => v.encode(w),
            ItemComponent::CreativeSlotLock => Ok(()),
            ItemComponent::EnchantmentGlintOverride(v) => v.encode(w),
            ItemComponent::IntangibleProjectile(v) => v.encode(w),
            ItemComponent::Food {
                nutrition,
                saturation_modifier,
                can_always_eat,
            } => {
                nutrition.encode(&mut *w)?;
                saturation_modifier.encode(&mut *w)?;
                can_always_eat.encode(w)
            }
            ItemComponent::Consumable {
                consume_seconds,
                animation,
                sound,
                has_consume_particles,
                effects,
            } => {
                consume_seconds.encode(&mut *w)?;
                animation.encode(&mut *w)?;
                sound.encode(&mut *w)?;
                has_consume_particles.encode(&mut *w)?;
                effects.encode(w)
            }
            ItemComponent::UseRemainder(v) => v.encode(w),
            ItemComponent::UseCooldown {
                seconds,
                cooldown_group,
            } => {
                seconds.encode(&mut *w)?;
                cooldown_group.encode(w)
            }
            ItemComponent::DamageResistant(v) => v.encode(w),
            ItemComponent::Tool {
                rules,
                default_mining_speed,
                damage_per_block,
                can_destroy_blocks_in_creative,
            } => {
                rules.encode(&mut *w)?;
                default_mining_speed.encode(&mut *w)?;
                damage_per_block.encode(&mut *w)?;
                can_destroy_blocks_in_creative.encode(w)
            }
            ItemComponent::Weapon {
                damage_per_attack,
                disable_blocking_for_seconds,
            } => {
                damage_per_attack.encode(&mut *w)?;
                disable_blocking_for_seconds.encode(w)
            }
            ItemComponent::Enchantable(v) => v.encode(w),
            ItemComponent::Equippable {
                slot,
                equip_sound,
                model,
                camera_overlay,
                allowed_entities,
                dispensable,
                swappable,
                damage_on_hurt,
            } => {
                slot.encode(&mut *w)?;
                equip_sound.encode(&mut *w)?;
                model.encode(&mut *w)?;
                camera_overlay.encode(&mut *w)?;
                allowed_entities.encode(&mut *w)?;
                dispensable.encode(&mut *w)?;
                swappable.encode(&mut *w)?;
                damage_on_hurt.encode(w)
            }
            ItemComponent::Repairable(v) => v.encode(w),
            ItemComponent::Glider => Ok(()),
            ItemComponent::TooltipStyle(v) => v.encode(w),
            ItemComponent::DeathProtection(v) => v.encode(w),
            ItemComponent::BlocksAttacks {
                block_delay_seconds,
                disable_cooldown_scale,
                damage_reductions,
                item_damage_threshold,
                item_damage_base,
                item_damage_factor,
                bypassed_by,
                block_sound,
                disable_sound,
            } => {
                block_delay_seconds.encode(&mut *w)?;
                disable_cooldown_scale.encode(&mut *w)?;
                damage_reductions.encode(&mut *w)?;
                item_damage_threshold.encode(&mut *w)?;
                item_damage_base.encode(&mut *w)?;
                item_damage_factor.encode(&mut *w)?;
                bypassed_by.encode(&mut *w)?;
                block_sound.encode(&mut *w)?;
                disable_sound.encode(w)
            }
            ItemComponent::StoredEnchantments {
                enchantments,
                show_in_tooltip,
            } => {
                enchantments.encode(&mut *w)?;
                show_in_tooltip.encode(w)
            }
            ItemComponent::DyedColor {
                color,
                show_in_tooltip,
            } => {
                color.encode(&mut *w)?;
                show_in_tooltip.encode(w)
            }
            ItemComponent::MapColor(v) => v.encode(w),
            ItemComponent::MapId(v) => v.encode(w),
            ItemComponent::MapDecorations(v) => v.encode(w),
            ItemComponent::MapPostProcessing(v) => v.encode(w),
            ItemComponent::ChargedProjectiles(v) => v.encode(w),
            ItemComponent::BundleContents(v) => v.encode(w),
            ItemComponent::PotionContents {
                potion_id,
                custom_color,
                custom_effects,
                custom_name,
            } => {
                potion_id.encode(&mut *w)?;
                custom_color.encode(&mut *w)?;
                custom_effects.encode(&mut *w)?;
                custom_name.encode(w)
            }
            ItemComponent::PotionDurationScale(v) => v.encode(w),
            ItemComponent::SuspiciousStewEffects(v) => v.encode(w),
            ItemComponent::WritableBookContent { pages } => pages.encode(w),
            ItemComponent::WrittenBookContent {
                raw_title,
                filtered_title,
                author,
                generation,
                pages,
                resolved,
            } => {
                raw_title.encode(&mut *w)?;
                filtered_title.encode(&mut *w)?;
                author.encode(&mut *w)?;
                generation.encode(&mut *w)?;
                pages.encode(&mut *w)?;
                resolved.encode(w)
            }
            ItemComponent::Trim {
                material,
                pattern,
                show_in_tooltip,
            } => {
                material.encode(&mut *w)?;
                pattern.encode(&mut *w)?;
                show_in_tooltip.encode(w)
            }
            ItemComponent::DebugStickState(v) => v.encode(w),
            ItemComponent::EntityData { id, data } => {
                id.encode(&mut *w)?;
                data.encode(w)
            }
            ItemComponent::BucketEntityData(v) => v.encode(w),
            ItemComponent::BlockEntityData { id, data } => {
                id.encode(&mut *w)?;
                data.encode(w)
            }
            ItemComponent::Instrument(v) => v.encode(w),
            ItemComponent::ProvidesTrimMaterial(v) => v.encode(w),
            ItemComponent::OminousBottleAmplifier(v) => v.encode(w),
            ItemComponent::JukeboxPlayable {
                song,
                show_in_tooltip,
            } => {
                song.encode(&mut *w)?;
                show_in_tooltip.encode(w)
            }
            ItemComponent::ProvidesBannerPatterns(v) => v.encode(w),
            ItemComponent::Recipes(v) => v.encode(w),
            ItemComponent::LodestoneTracker { target, tracked } => {
                target.encode(&mut *w)?;
                tracked.encode(w)
            }
            ItemComponent::FireworkExplosion(v) => v.encode(w),
            ItemComponent::Fireworks {
                flight_duration,
                explosions,
            } => {
                flight_duration.encode(&mut *w)?;
                explosions.encode(w)
            }
            ItemComponent::Profile(v) => v.encode(w),
            ItemComponent::NoteBlockSound(v) => v.encode(w),
            ItemComponent::BannerPatterns(v) => v.encode(w),
            ItemComponent::BaseColor(v) => v.encode(w),
            ItemComponent::PotDecorations(v) => v.encode(w),
            ItemComponent::Container(v) => v.encode(w),
            ItemComponent::BlockState(v) => v.encode(w),
            ItemComponent::Bees(v) => v.encode(w),
            ItemComponent::Lock(v) => v.encode(w),
            ItemComponent::ContainerLoot(v) => v.encode(w),
            ItemComponent::BreakSound(v) => v.encode(w),
            ItemComponent::VillagerVariant(v) => v.encode(w),
            ItemComponent::WolfVariant(v) => v.encode(w),
            ItemComponent::WolfSoundVariant(v) => v.encode(w),
            ItemComponent::WolfCollar(v) => v.encode(w),
            ItemComponent::FoxVariant(v) => v.encode(w),
            ItemComponent::SalmonSize(v) => v.encode(w),
            ItemComponent::ParrotVariant(v) => v.encode(w),
            ItemComponent::TropicalFishPattern(v) => v.encode(w),
            ItemComponent::TropicalFishBaseColor(v) => v.encode(w),
            ItemComponent::TropicalFishPatternColor(v) => v.encode(w),
            ItemComponent::MooshroomVariant(v) => v.encode(w),
            ItemComponent::RabbitVariant(v) => v.encode(w),
            ItemComponent::PigVariant(v) => v.encode(w),
            ItemComponent::CowVariant(v) => v.encode(w),
            ItemComponent::ChickenVariant(v) => v.encode(w),
            ItemComponent::FrogVariant(v) => v.encode(w),
            ItemComponent::HorseVariant(v) => v.encode(w),
            ItemComponent::PaintingVariant(v) => v.encode(w),
            ItemComponent::LlamaVariant(v) => v.encode(w),
            ItemComponent::AxolotlVariant(v) => v.encode(w),
            ItemComponent::CatVariant(v) => v.encode(w),
            ItemComponent::CatCollar(v) => v.encode(w),
            ItemComponent::SheepColor(v) => v.encode(w),
            ItemComponent::ShulkerColor(v) => v.encode(w),
        }
    }
}
impl<'a> Decode<'a> for ItemStack {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        decode_item_stack_recursive(r, 0, false)
    }
}

pub(crate) fn decode_item_stack_recursive(
    r: &mut &[u8],
    depth: usize,
    prefixed: bool,
) -> anyhow::Result<ItemStack> {
    if depth > MAX_RECURSION_DEPTH {
        return Err(anyhow::anyhow!("ItemStack recursion limit exceeded"));
    }

    let count = VarInt::decode(r)?.0;
    if count <= 0 {
        return Ok(ItemStack::EMPTY);
    }
    let item = ItemKind::decode(r)?;

    let mut components = item.default_components();

    // Decode counts
    let added_count = VarInt::decode(r)?.0;
    let removed_count = VarInt::decode(r)?.0;

    // Decode Added Components
    for _ in 0..added_count {
        let id = VarInt::decode(r)?.0 as usize;
        if id >= NUM_ITEM_COMPONENTS {
            return Err(anyhow::anyhow!("Invalid item component ID: {id}"));
        }

        let _prefix = if prefixed {
            Some(VarInt::decode(r)?)
        } else {
            None
        }; // TODO: Use prefix?

        let component = decode_item_component(r, id, depth)?;
        let hash = component.hash();
        components[id] = Patchable::Added((Box::new(component), hash));
    }

    // Decode Removed Components
    for _ in 0..removed_count {
        let id = VarInt::decode(r)?.0 as usize;
        if id >= NUM_ITEM_COMPONENTS {
            return Err(anyhow::anyhow!("Invalid item component ID: {id}"));
        }
        components[id] = Patchable::Removed;
    }

    Ok(ItemStack {
        item,
        count: count as i8,
        components,
    })
}

fn decode_block_predicate(r: &mut &[u8], depth: usize) -> anyhow::Result<BlockPredicate> {
    Ok(BlockPredicate {
        blocks: Decode::decode(r)?,
        properties: Decode::decode(r)?,
        nbt: Decode::decode(r)?,
        exact_components: {
            // Vec = |len|item*len|
            let length = VarInt::decode(r)?.0 as usize;
            let mut vec = Vec::with_capacity(cautious_capacity::<ExactComponentMatcher>(length));
            for _ in 0..length {
                let component_type = VarInt::decode(r)?;
                let component_data =
                    decode_item_component(r, component_type.0 as usize, depth + 1)?;
                vec.push(ExactComponentMatcher {
                    component_type,
                    component_data,
                });
            }
            vec
        },
        partial_components: Decode::decode(r)?,
    })
}

fn decode_item_component(r: &mut &[u8], id: usize, depth: usize) -> anyhow::Result<ItemComponent> {
    Ok(match id {
        0 => ItemComponent::CustomData(Decode::decode(r)?),
        1 => ItemComponent::MaxStackSize(Decode::decode(r)?),
        2 => ItemComponent::MaxDamage(Decode::decode(r)?),
        3 => ItemComponent::Damage(Decode::decode(r)?),
        4 => ItemComponent::Unbreakable,
        5 => ItemComponent::CustomName(Decode::decode(r)?),
        6 => ItemComponent::ItemName(Decode::decode(r)?),
        7 => ItemComponent::ItemModel(Decode::decode(r)?),
        8 => ItemComponent::Lore(Decode::decode(r)?),
        9 => ItemComponent::Rarity(Decode::decode(r)?),
        10 => ItemComponent::Enchantments(Decode::decode(r)?),
        11 => ItemComponent::CanPlaceOn({
            let count = VarInt::decode(r)?.0;
            let mut items = Vec::with_capacity(cautious_capacity::<BlockPredicate>(count as usize));
            for _ in 0..count {
                items.push(decode_block_predicate(r, depth)?);
            }
            items
        }),
        12 => ItemComponent::CanBreak({
            let count = VarInt::decode(r)?.0;
            let mut items = Vec::with_capacity(cautious_capacity::<BlockPredicate>(count as usize));
            for _ in 0..count {
                items.push(decode_block_predicate(r, depth)?);
            }
            items
        }),
        13 => ItemComponent::AttributeModifiers {
            modifiers: Decode::decode(r)?,
        },
        14 => ItemComponent::CustomModelData {
            floats: Decode::decode(r)?,
            flags: Decode::decode(r)?,
            strings: Decode::decode(r)?,
            colors: Decode::decode(r)?,
        },
        15 => ItemComponent::TooltipDisplay {
            hide_tooltip: Decode::decode(r)?,
            hidden_components: Decode::decode(r)?,
        },
        16 => ItemComponent::RepairCost(Decode::decode(r)?),
        17 => ItemComponent::CreativeSlotLock,
        18 => ItemComponent::EnchantmentGlintOverride(Decode::decode(r)?),
        19 => ItemComponent::IntangibleProjectile(Decode::decode(r)?),
        20 => ItemComponent::Food {
            nutrition: Decode::decode(r)?,
            saturation_modifier: Decode::decode(r)?,
            can_always_eat: Decode::decode(r)?,
        },
        21 => ItemComponent::Consumable {
            consume_seconds: Decode::decode(r)?,
            animation: Decode::decode(r)?,
            sound: Decode::decode(r)?,
            has_consume_particles: Decode::decode(r)?,
            effects: Decode::decode(r)?,
        },
        22 => {
            ItemComponent::UseRemainder(Box::new(decode_item_stack_recursive(r, depth + 1, false)?))
        }
        23 => ItemComponent::UseCooldown {
            seconds: Decode::decode(r)?,
            cooldown_group: Decode::decode(r)?,
        },
        24 => ItemComponent::DamageResistant(Decode::decode(r)?),
        25 => ItemComponent::Tool {
            rules: Decode::decode(r)?,
            default_mining_speed: Decode::decode(r)?,
            damage_per_block: Decode::decode(r)?,
            can_destroy_blocks_in_creative: Decode::decode(r)?,
        },
        26 => ItemComponent::Weapon {
            damage_per_attack: Decode::decode(r)?,
            disable_blocking_for_seconds: Decode::decode(r)?,
        },
        27 => ItemComponent::Enchantable(Decode::decode(r)?),
        28 => ItemComponent::Equippable {
            slot: Decode::decode(r)?,
            equip_sound: Decode::decode(r)?,
            model: Decode::decode(r)?,
            camera_overlay: Decode::decode(r)?,
            allowed_entities: Decode::decode(r)?,
            dispensable: Decode::decode(r)?,
            swappable: Decode::decode(r)?,
            damage_on_hurt: Decode::decode(r)?,
        },
        29 => ItemComponent::Repairable(Decode::decode(r)?),
        30 => ItemComponent::Glider,
        31 => ItemComponent::TooltipStyle(Decode::decode(r)?),
        32 => ItemComponent::DeathProtection(Decode::decode(r)?),
        33 => ItemComponent::BlocksAttacks {
            block_delay_seconds: Decode::decode(r)?,
            disable_cooldown_scale: Decode::decode(r)?,
            damage_reductions: Decode::decode(r)?,
            item_damage_threshold: Decode::decode(r)?,
            item_damage_base: Decode::decode(r)?,
            item_damage_factor: Decode::decode(r)?,
            bypassed_by: Decode::decode(r)?,
            block_sound: Decode::decode(r)?,
            disable_sound: Decode::decode(r)?,
        },
        34 => ItemComponent::StoredEnchantments {
            enchantments: Decode::decode(r)?,
            show_in_tooltip: Decode::decode(r)?,
        },
        35 => ItemComponent::DyedColor {
            color: Decode::decode(r)?,
            show_in_tooltip: Decode::decode(r)?,
        },
        36 => ItemComponent::MapColor(Decode::decode(r)?),
        37 => ItemComponent::MapId(Decode::decode(r)?),
        38 => ItemComponent::MapDecorations(Decode::decode(r)?),
        39 => ItemComponent::MapPostProcessing(Decode::decode(r)?),
        40 => {
            let count = VarInt::decode(r)?.0;
            let mut items = Vec::with_capacity(cautious_capacity::<ItemStack>(count as usize));
            for _ in 0..count {
                items.push(decode_item_stack_recursive(r, depth + 1, false)?);
            }
            ItemComponent::ChargedProjectiles(items)
        }
        41 => {
            let count = VarInt::decode(r)?.0;
            let mut items = Vec::with_capacity(cautious_capacity::<ItemStack>(count as usize));
            for _ in 0..count {
                items.push(decode_item_stack_recursive(r, depth + 1, false)?);
            }
            ItemComponent::BundleContents(items)
        }
        42 => ItemComponent::PotionContents {
            potion_id: Decode::decode(r)?,
            custom_color: Decode::decode(r)?,
            custom_effects: Decode::decode(r)?,
            custom_name: Decode::decode(r)?,
        },
        43 => ItemComponent::PotionDurationScale(Decode::decode(r)?),
        44 => ItemComponent::SuspiciousStewEffects(Decode::decode(r)?),
        45 => ItemComponent::WritableBookContent {
            pages: Decode::decode(r)?,
        },
        46 => ItemComponent::WrittenBookContent {
            raw_title: Decode::decode(r)?,
            filtered_title: Decode::decode(r)?,
            author: Decode::decode(r)?,
            generation: Decode::decode(r)?,
            pages: Decode::decode(r)?,
            resolved: Decode::decode(r)?,
        },
        47 => ItemComponent::Trim {
            material: Decode::decode(r)?,
            pattern: Decode::decode(r)?,
            show_in_tooltip: Decode::decode(r)?,
        },
        48 => ItemComponent::DebugStickState(Decode::decode(r)?),
        49 => ItemComponent::EntityData {
            id: Decode::decode(r)?,
            data: Decode::decode(r)?,
        },
        50 => ItemComponent::BucketEntityData(Decode::decode(r)?),
        51 => ItemComponent::BlockEntityData {
            id: Decode::decode(r)?,
            data: Decode::decode(r)?,
        },
        52 => ItemComponent::Instrument(Decode::decode(r)?),
        53 => ItemComponent::ProvidesTrimMaterial(Decode::decode(r)?),
        54 => ItemComponent::OminousBottleAmplifier(Decode::decode(r)?),
        55 => ItemComponent::JukeboxPlayable {
            song: Decode::decode(r)?,
            show_in_tooltip: Decode::decode(r)?,
        },
        56 => ItemComponent::ProvidesBannerPatterns(Decode::decode(r)?),
        57 => ItemComponent::Recipes(Decode::decode(r)?),
        58 => ItemComponent::LodestoneTracker {
            target: Decode::decode(r)?,
            tracked: Decode::decode(r)?,
        },
        59 => ItemComponent::FireworkExplosion(Decode::decode(r)?),
        60 => ItemComponent::Fireworks {
            flight_duration: Decode::decode(r)?,
            explosions: Decode::decode(r)?,
        },
        61 => ItemComponent::Profile(Decode::decode(r)?),
        62 => ItemComponent::NoteBlockSound(Decode::decode(r)?),
        63 => ItemComponent::BannerPatterns(Decode::decode(r)?),
        64 => ItemComponent::BaseColor(Decode::decode(r)?),
        65 => ItemComponent::PotDecorations(Decode::decode(r)?),
        66 => {
            let count = VarInt::decode(r)?.0;
            let mut items = Vec::with_capacity(cautious_capacity::<ItemStack>(count as usize));
            for _ in 0..count {
                items.push(decode_item_stack_recursive(r, depth + 1, false)?);
            }
            ItemComponent::Container(items)
        }
        67 => ItemComponent::BlockState(Decode::decode(r)?),
        68 => ItemComponent::Bees(Decode::decode(r)?),
        69 => ItemComponent::Lock(Decode::decode(r)?),
        70 => ItemComponent::ContainerLoot(Decode::decode(r)?),
        71 => ItemComponent::BreakSound(Decode::decode(r)?),
        72 => ItemComponent::VillagerVariant(Decode::decode(r)?),
        73 => ItemComponent::WolfVariant(Decode::decode(r)?),
        74 => ItemComponent::WolfSoundVariant(Decode::decode(r)?),
        75 => ItemComponent::WolfCollar(Decode::decode(r)?),
        76 => ItemComponent::FoxVariant(Decode::decode(r)?),
        77 => ItemComponent::SalmonSize(Decode::decode(r)?),
        78 => ItemComponent::ParrotVariant(Decode::decode(r)?),
        79 => ItemComponent::TropicalFishPattern(Decode::decode(r)?),
        80 => ItemComponent::TropicalFishBaseColor(Decode::decode(r)?),
        81 => ItemComponent::TropicalFishPatternColor(Decode::decode(r)?),
        82 => ItemComponent::MooshroomVariant(Decode::decode(r)?),
        83 => ItemComponent::RabbitVariant(Decode::decode(r)?),
        84 => ItemComponent::PigVariant(Decode::decode(r)?),
        85 => ItemComponent::CowVariant(Decode::decode(r)?),
        86 => ItemComponent::ChickenVariant(Decode::decode(r)?),
        87 => ItemComponent::FrogVariant(Decode::decode(r)?),
        88 => ItemComponent::HorseVariant(Decode::decode(r)?),
        89 => ItemComponent::PaintingVariant(Decode::decode(r)?),
        90 => ItemComponent::LlamaVariant(Decode::decode(r)?),
        91 => ItemComponent::AxolotlVariant(Decode::decode(r)?),
        92 => ItemComponent::CatVariant(Decode::decode(r)?),
        93 => ItemComponent::CatCollar(Decode::decode(r)?),
        94 => ItemComponent::SheepColor(Decode::decode(r)?),
        95 => ItemComponent::ShulkerColor(Decode::decode(r)?),
        _ => return Err(anyhow::anyhow!("Unknown ItemComponent ID: {id}")),
    })
}

// Encode for HashedItemStack as described in "Hashed Format"
impl Encode for HashedItemStack {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        if self.is_empty() {
            false.encode(&mut w)
        } else {
            true.encode(&mut w)?;
            self.item.encode(&mut w)?;
            VarInt(i32::from(self.count)).encode(&mut w)?;

            let mut added = Vec::new();
            let mut removed = Vec::new();

            for (i, c) in self.components.iter().enumerate() {
                match c {
                    Patchable::Added(((), hash)) => added.push((i, hash)),
                    Patchable::Removed => removed.push(i),
                    _ => {}
                }
            }

            VarInt(added.len() as i32).encode(&mut w)?;
            for (id, hash) in added {
                VarInt(id as i32).encode(&mut w)?;
                hash.encode(&mut w)?;
            }

            VarInt(removed.len() as i32).encode(&mut w)?;
            for id in removed {
                VarInt(id as i32).encode(&mut w)?;
            }

            Ok(())
        }
    }
}
impl Decode<'_> for HashedItemStack {
    fn decode(r: &mut &'_ [u8]) -> anyhow::Result<Self> {
        let has_item = bool::decode(r)?;
        if !has_item {
            Ok(Self::EMPTY)
        } else {
            let item = ItemKind::decode(r)?;
            let item_count = VarInt::decode(r)?;

            let mut components = [Patchable::None; NUM_ITEM_COMPONENTS];

            let components_added: Vec<(VarInt, i32)> = Vec::decode(r)?;
            let components_removed: Vec<VarInt> = Vec::decode(r)?;

            for (id, hash) in components_added {
                let id = id.0 as usize;
                if id >= NUM_ITEM_COMPONENTS {
                    return Err(anyhow::anyhow!("Invalid item component ID: {id}"));
                }
                components[id] = Patchable::Added(((), hash));
            }

            for id in components_removed {
                let id = id.0 as usize;
                if id >= NUM_ITEM_COMPONENTS {
                    return Err(anyhow::anyhow!("Invalid item component ID: {id}"));
                }
                components[id] = Patchable::Removed;
            }

            Ok(Self {
                item,
                count: item_count.0 as i8,
                components,
            })
        }
    }
}

pub trait ItemKindExt {
    /// Returns the default components for the [`ItemKind`].
    // The reason we use two lifetimes is to tell the compiler that
    // the ref self is not the same as the ref for the returned ItemComponents
    // so we can drop Self
    fn default_components(&self) -> [Patchable<Box<ItemComponent>>; NUM_ITEM_COMPONENTS];
}

impl ItemKindExt for ItemKind {
    fn default_components(&self) -> [Patchable<Box<ItemComponent>>; NUM_ITEM_COMPONENTS] {
        //     let ser_default_components = self.ser_components();
        //     let mut components = [const { None }; NUM_ITEM_COMPONENTS];

        //     for component in ser_default_components {
        //         let item_component = ItemComponent::from_serialized(component);
        //         let id = item_component.id() as usize;
        //         components[id] = Some(Box::new(item_component));
        //     }

        //     components
        // }

        [const { Patchable::None }; NUM_ITEM_COMPONENTS]
    }
}

#[cfg(test)]
mod tests {
    use valence_ident::ident;
    use valence_nbt::{Compound, List};
    use valence_text::Text;

    use super::*;

    // --- Helpers ---

    fn create_test_stack(item: ItemKind, count: i8) -> ItemStack {
        ItemStack::new(item, count)
    }

    fn roundtrip<T: Encode + for<'a> Decode<'a> + PartialEq + std::fmt::Debug>(val: &T) {
        let mut buf = Vec::new();
        val.encode(&mut buf).expect("Failed to encode");
        let mut slice = buf.as_slice();
        let decoded = T::decode(&mut slice).expect("Failed to decode");
        assert_eq!(val, &decoded, "Roundtrip failed equality check");
        assert!(slice.is_empty(), "Buffer not fully consumed");
    }

    // --- Patchable Tests ---

    #[test]
    fn test_patchable_logic() {
        let p_added = Patchable::Added((Box::new(10), 123));
        let p_default = Patchable::Default(Box::new(5));
        let p_removed: Patchable<Box<i32>> = Patchable::Removed;
        let p_none: Patchable<Box<i32>> = Patchable::None;

        assert_eq!(p_added.to_option_ref().map(|v| **v), Some(10));
        assert_eq!(p_default.to_option_ref().map(|v| **v), Some(5));
        assert_eq!(p_removed.to_option_ref(), None);
        assert_eq!(p_none.to_option_ref(), None);
    }

    // --- ItemStack Logical Tests ---

    #[test]
    fn test_item_stack_empty() {
        let empty = ItemStack::EMPTY;
        assert!(empty.is_empty());

        let air = ItemStack::new(ItemKind::Air, 1);
        assert!(air.is_empty());

        let stack = ItemStack::new(ItemKind::Diamond, 0);
        assert!(stack.is_empty());

        let stack = ItemStack::new(ItemKind::Diamond, -1);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_component_insertion_and_removal() {
        let mut stack = create_test_stack(ItemKind::Diamond, 1);

        // Test Insert
        let custom_name = ItemComponent::CustomName(Text::from("Test Item").into());
        stack.insert_component(custom_name.clone());

        assert_eq!(stack.get_component(5_usize), Some(&custom_name));
        assert_eq!(stack.components().len(), 1);

        // Test Remove
        let removed = stack.remove_component(5_usize);
        assert_eq!(removed, Some(custom_name));
        assert_eq!(stack.get_component(5_usize), None);

        // Ensure "Removed" patch is applied (important for serialization)
        assert!(matches!(stack.components[5], Patchable::Removed));
    }

    // --- Serialization Roundtrips ---

    #[test]
    fn test_serialization_basic_stack() {
        let stack = ItemStack::new(ItemKind::Stone, 32);
        roundtrip(&stack);
    }

    #[test]
    fn test_serialization_with_complex_components() {
        let mut stack = ItemStack::new(ItemKind::DiamondSword, 1);

        // Add multiple types of components
        stack.insert_component(ItemComponent::Damage(VarInt(50)));
        stack.insert_component(ItemComponent::Rarity(Rarity::Epic));

        // Test vec-based components (Enchantments)
        stack.insert_component(ItemComponent::Enchantments(vec![(
            RegistryId::new(1),
            VarInt(5),
        )]));

        roundtrip(&stack);
    }

    #[test]
    fn test_serialization_removed_components() {
        // Start with a stack that has a component, then remove it
        let mut stack = ItemStack::new(ItemKind::Diamond, 1);
        stack.insert_component(ItemComponent::Unbreakable);
        stack.remove_component(ItemComponent::Unbreakable.id() as usize);

        roundtrip(&stack);
    }

    #[test]
    fn test_mode_pair_serialization() {
        let m0 = ModePair::<String, RegistryId>::Mode0("minecraft:standard".to_string());
        let m1 = ModePair::<String, RegistryId>::Mode1(RegistryId::new(1));

        roundtrip(&m0);
        roundtrip(&m1);
    }

    #[test]
    fn test_property_value_serialization() {
        let exact = PropertyValue::Exact("true".into());
        let min_max = PropertyValue::MinMax {
            min: "1".into(),
            max: "5".into(),
        };

        roundtrip(&exact);
        roundtrip(&min_max);
    }

    // --- Recursion and Nested Components ---

    #[test]
    fn test_nested_container_serialization() {
        let mut inner_stack = ItemStack::new(ItemKind::Apple, 1);
        inner_stack.insert_component(ItemComponent::ItemName(Text::from("Inner").into()));

        let mut outer_stack = ItemStack::new(ItemKind::Chest, 1);
        outer_stack.insert_component(ItemComponent::Container(vec![inner_stack]));

        roundtrip(&outer_stack);
    }

    #[test]
    fn test_recursion_limit() {
        let mut buf = Vec::new();

        // Helper to write a recursive bundle structure manually
        fn write_recursive_bundle(mut w: &mut Vec<u8>, depth: usize) {
            VarInt(1).encode(&mut *w).unwrap(); // Count
            ItemKind::Bundle.encode(&mut *w).unwrap(); // Item

            VarInt(1).encode(&mut *w).unwrap(); // Added components count
            VarInt(41).encode(&mut *w).unwrap(); // Component ID: BundleContents

            if depth > 0 {
                VarInt(1).encode(&mut *w).unwrap(); // Nested list length
                write_recursive_bundle(w, depth - 1);
            } else {
                VarInt(0).encode(&mut *w).unwrap(); // Empty nested list
            }

            VarInt(0).encode(w).unwrap(); // Removed components count
        }

        write_recursive_bundle(&mut buf, 20); // 20 > 16

        let mut slice = buf.as_slice();
        let result = ItemStack::decode(&mut slice);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("recursion limit exceeded"));
    }

    // --- HashedItemStack Tests ---

    #[test]
    fn test_hashed_item_stack_roundtrip() {
        let mut hashed = HashedItemStack::EMPTY;
        hashed.item = ItemKind::IronIngot;
        hashed.count = 10;
        // In real use, these would be crc hashes
        hashed.components[1] = Patchable::Added(((), 123456));

        roundtrip(&hashed);
    }

    #[test]
    fn test_hashed_item_stack_empty() {
        let hashed = HashedItemStack::EMPTY;
        let mut buf = Vec::new();
        hashed.encode(&mut buf).unwrap();

        let mut slice = buf.as_slice();
        let decoded = HashedItemStack::decode(&mut slice).unwrap();
        assert!(decoded.is_empty());
    }

    // --- Edge Cases ---

    #[test]
    fn test_invalid_component_id() {
        let mut buf = Vec::new();
        VarInt(1).encode(&mut buf).unwrap(); // Count
        ItemKind::Stone.encode(&mut buf).unwrap(); // Item
        VarInt(1).encode(&mut buf).unwrap(); // Added count
        VarInt(999).encode(&mut buf).unwrap(); // INVALID ID

        let mut slice = buf.as_slice();
        let result = ItemStack::decode(&mut slice);
        assert!(result.is_err());
    }

    #[test]
    fn test_all_item_component_ids() {
        let mut ids = std::collections::HashSet::new();
        let components = vec![
            ItemComponent::CustomData(Compound::default()),
            ItemComponent::MaxStackSize(VarInt(64)),
            ItemComponent::Unbreakable,
            ItemComponent::Glider,
            ItemComponent::ShulkerColor(DyeColor::Black),
        ];

        for comp in components {
            let id = comp.id();
            assert!(id < NUM_ITEM_COMPONENTS as u32);
            ids.insert(id);
        }

        assert!(ids.contains(&4));
        assert!(ids.contains(&30));
    }

    #[test]
    fn test_food_component_serialization() {
        let food = ItemComponent::Food {
            nutrition: VarInt(4),
            saturation_modifier: 0.5,
            can_always_eat: true,
        };
        let mut stack = ItemStack::new(ItemKind::Apple, 1);
        stack.insert_component(food);
        roundtrip(&stack);
    }

    #[test]
    fn test_attribute_modifiers_serialization() {
        let modifier = AttributeModifier {
            attribute_id: RegistryId::new(0),
            modifier_id: ident!("test_mod").into(),
            value: 5.0,
            operation: EntityAttributeOperation::Add,
            slot: AttributeSlot::MainHand,
        };

        let comp = ItemComponent::AttributeModifiers {
            modifiers: vec![modifier],
        };

        let mut stack = ItemStack::new(ItemKind::NetheriteSword, 1);
        stack.insert_component(comp);
        roundtrip(&stack);
    }
}
