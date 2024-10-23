use std::borrow::Cow;
use std::io::Write;

use anyhow::bail;
use valence_generated::block::BlockState;
use valence_math::{DVec3, Vec3};

use crate::{BlockPos, Decode, Encode, ItemStack, Packet, VarInt};

#[derive(Clone, Debug, Packet)]
pub struct LevelParticlesS2c<'a> {
    pub long_distance: bool,
    pub position: DVec3,
    pub offset: Vec3,
    pub max_speed: f32,
    pub count: i32,
    pub particle: Cow<'a, Particle>,
}

impl Encode for LevelParticlesS2c<'_> {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        self.long_distance.encode(&mut w)?;
        self.position.encode(&mut w)?;
        self.offset.encode(&mut w)?;
        self.max_speed.encode(&mut w)?;
        self.count.encode(&mut w)?;

        VarInt(self.particle.id()).encode(&mut w)?;
        self.particle.as_ref().encode(w)
    }
}

impl<'a> Decode<'a> for LevelParticlesS2c<'a> {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        let long_distance = bool::decode(r)?;
        let position = Decode::decode(r)?;
        let offset = Decode::decode(r)?;
        let max_speed = f32::decode(r)?;
        let particle_count = i32::decode(r)?;
        let particle_id = VarInt::decode(r)?.0;

        Ok(Self {
            particle: Cow::Owned(Particle::decode_with_id(particle_id, r)?),
            long_distance,
            position,
            offset,
            max_speed,
            count: particle_count,
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Particle {
    AngryVillager,
    Block(BlockState),
    BlockMarker(BlockState),
    Bubble,
    Cloud,
    Crit,
    DamageIndicator,
    DragonBreath,
    DrippingLava,
    FallingLava,
    LandingLava,
    DrippingWater,
    FallingWater,
    Dust {
        rgb: Vec3,
        scale: f32,
    },
    DustColorTransition {
        from_rgb: Vec3,
        to_rgb: Vec3,
        scale: f32,
    },
    Effect,
    ElderGuardian,
    EnchantedHit,
    Enchant,
    EndRod,
    EntityEffect {
        color: i32,
    },
    ExplosionEmitter,
    Explosion,
    Gust,
    SmallGust,
    GustEmitterLarge,
    GustEmitterSmall,
    SonicBoom,
    FallingDust(BlockState),
    Firework,
    Fishing,
    Flame,
    Infested,
    CherryLeaves,
    SculkSoul,
    SculkCharge {
        roll: f32,
    },
    SculkChargePop,
    SoulFireFlame,
    Soul,
    Flash,
    HappyVillager,
    Composter,
    Heart,
    InstantEffect,
    Item(ItemStack),
    /// The 'Block' variant of the 'Vibration' particle
    VibrationBlock {
        block_pos: BlockPos,
        ticks: i32,
    },
    /// The 'Entity' variant of the 'Vibration' particle
    VibrationEntity {
        entity_id: i32,
        entity_eye_height: f32,
        ticks: i32,
    },
    ItemSlime,
    ItemCobweb,
    ItemSnowball,
    LargeSmoke,
    Lava,
    Mycelium,
    Note,
    Poof,
    Portal,
    Rain,
    Smoke,
    WhiteSmoke,
    Sneeze,
    Spit,
    SquidInk,
    SweepAttack,
    TotemOfUndying,
    Underwater,
    Splash,
    Witch,
    BubblePop,
    CurrentDown,
    BubbleColumnUp,
    Nautilus,
    Dolphin,
    CampfireCosySmoke,
    CampfireSignalSmoke,
    DrippingHoney,
    FallingHoney,
    LandingHoney,
    FallingNectar,
    FallingSporeBlossom,
    Ash,
    CrimsonSpore,
    WarpedSpore,
    SporeBlossomAir,
    DrippingObsidianTear,
    FallingObsidianTear,
    LandingObsidianTear,
    ReversePortal,
    WhiteAsh,
    SmallFlame,
    Snowflake,
    DrippingDripstoneLava,
    FallingDripstoneLava,
    DrippingDripstoneWater,
    FallingDripstoneWater,
    GlowSquidInk,
    Glow,
    WaxOn,
    WaxOff,
    ElectricSpark,
    Scrape,
    Shriek {
        delay: i32,
    },
    EggCrack,
    DustPlume,
    TrialSpawnerDetection,
    TrialSpawnerDetectionOminous,
    VaultConnection,
    DustPillar(BlockState),
    OminousSpawning,
    RaidOmen,
    TrialOmen,
}

impl Particle {
    pub const fn id(&self) -> i32 {
        match self {
            Particle::AngryVillager => 0,
            Particle::Block(_) => 1,
            Particle::BlockMarker(_) => 2,
            Particle::Bubble => 3,
            Particle::Cloud => 4,
            Particle::Crit => 5,
            Particle::DamageIndicator => 6,
            Particle::DragonBreath => 7,
            Particle::DrippingLava => 8,
            Particle::FallingLava => 9,
            Particle::LandingLava => 10,
            Particle::DrippingWater => 11,
            Particle::FallingWater => 12,
            Particle::Dust { .. } => 13,
            Particle::DustColorTransition { .. } => 14,
            Particle::Effect => 15,
            Particle::ElderGuardian => 16,
            Particle::EnchantedHit => 17,
            Particle::Enchant => 18,
            Particle::EndRod => 19,
            Particle::EntityEffect { .. } => 20,
            Particle::ExplosionEmitter => 21,
            Particle::Explosion => 22,
            Particle::Gust => 23,
            Particle::SmallGust => 24,
            Particle::GustEmitterLarge => 25,
            Particle::GustEmitterSmall => 26,
            Particle::SonicBoom => 27,
            Particle::FallingDust(_) => 28,
            Particle::Firework => 29,
            Particle::Fishing => 30,
            Particle::Flame => 31,
            Particle::Infested => 32,
            Particle::CherryLeaves => 33,
            Particle::SculkSoul => 34,
            Particle::SculkCharge { .. } => 35,
            Particle::SculkChargePop => 36,
            Particle::SoulFireFlame => 37,
            Particle::Soul => 38,
            Particle::Flash => 39,
            Particle::HappyVillager => 40,
            Particle::Composter => 41,
            Particle::Heart => 42,
            Particle::InstantEffect => 43,
            Particle::Item { .. } => 44,
            Particle::VibrationBlock { .. } => 45,
            Particle::VibrationEntity { .. } => 45,
            Particle::ItemSlime => 46,
            Particle::ItemCobweb => 47,
            Particle::ItemSnowball => 48,
            Particle::LargeSmoke => 49,
            Particle::Lava => 50,
            Particle::Mycelium => 51,
            Particle::Note => 52,
            Particle::Poof => 53,
            Particle::Portal => 54,
            Particle::Rain => 55,
            Particle::Smoke => 56,
            Particle::WhiteSmoke => 57,
            Particle::Sneeze => 58,
            Particle::Spit => 59,
            Particle::SquidInk => 60,
            Particle::SweepAttack => 61,
            Particle::TotemOfUndying => 62,
            Particle::Underwater => 63,
            Particle::Splash => 64,
            Particle::Witch => 65,
            Particle::BubblePop => 66,
            Particle::CurrentDown => 67,
            Particle::BubbleColumnUp => 68,
            Particle::Nautilus => 69,
            Particle::Dolphin => 70,
            Particle::CampfireCosySmoke => 71,
            Particle::CampfireSignalSmoke => 72,
            Particle::DrippingHoney => 73,
            Particle::FallingHoney => 74,
            Particle::LandingHoney => 75,
            Particle::FallingNectar => 76,
            Particle::FallingSporeBlossom => 77,
            Particle::Ash => 78,
            Particle::CrimsonSpore => 79,
            Particle::WarpedSpore => 80,
            Particle::SporeBlossomAir => 81,
            Particle::DrippingObsidianTear => 82,
            Particle::FallingObsidianTear => 83,
            Particle::LandingObsidianTear => 84,
            Particle::ReversePortal => 85,
            Particle::WhiteAsh => 86,
            Particle::SmallFlame => 87,
            Particle::Snowflake => 88,
            Particle::DrippingDripstoneLava => 89,
            Particle::FallingDripstoneLava => 90,
            Particle::DrippingDripstoneWater => 91,
            Particle::FallingDripstoneWater => 92,
            Particle::GlowSquidInk => 93,
            Particle::Glow => 94,
            Particle::WaxOn => 95,
            Particle::WaxOff => 96,
            Particle::ElectricSpark => 97,
            Particle::Scrape => 98,
            Particle::Shriek { .. } => 99,
            Particle::EggCrack => 100,
            Particle::DustPlume => 101,
            Particle::TrialSpawnerDetection => 102,
            Particle::TrialSpawnerDetectionOminous => 103,
            Particle::VaultConnection => 104,
            Particle::DustPillar(_) => 105,
            Particle::OminousSpawning => 106,
            Particle::RaidOmen => 107,
            Particle::TrialOmen => 108,
        }
    }

    /// Decodes the particle assuming the given particle ID.
    pub fn decode_with_id(particle_id: i32, r: &mut &[u8]) -> anyhow::Result<Self> {
        Ok(match particle_id {
            0 => Particle::AngryVillager,
            1 => Particle::Block(BlockState::decode(r)?),
            2 => Particle::BlockMarker(BlockState::decode(r)?),
            3 => Particle::Bubble,
            4 => Particle::Cloud,
            5 => Particle::Crit,
            6 => Particle::DamageIndicator,
            7 => Particle::DragonBreath,
            8 => Particle::DrippingLava,
            9 => Particle::FallingLava,
            10 => Particle::LandingLava,
            11 => Particle::DrippingWater,
            12 => Particle::FallingWater,
            13 => Particle::Dust {
                rgb: Decode::decode(r)?,
                scale: Decode::decode(r)?,
            },
            14 => Particle::DustColorTransition {
                from_rgb: Decode::decode(r)?,
                scale: Decode::decode(r)?,
                to_rgb: Decode::decode(r)?,
            },
            15 => Particle::Effect,
            16 => Particle::ElderGuardian,
            17 => Particle::EnchantedHit,
            18 => Particle::Enchant,
            19 => Particle::EndRod,
            20 => Particle::EntityEffect {
                color: Decode::decode(r)?,
            },
            21 => Particle::ExplosionEmitter,
            22 => Particle::Explosion,
            23 => Particle::Gust,
            24 => Particle::SmallGust,
            25 => Particle::GustEmitterLarge,
            26 => Particle::GustEmitterSmall,
            27 => Particle::SonicBoom,
            28 => Particle::FallingDust(BlockState::decode(r)?),
            29 => Particle::Firework,
            30 => Particle::Fishing,
            31 => Particle::Flame,
            32 => Particle::Infested,
            33 => Particle::CherryLeaves,
            34 => Particle::SculkSoul,
            35 => Particle::SculkCharge {
                roll: f32::decode(r)?,
            },
            36 => Particle::SculkChargePop,
            37 => Particle::SoulFireFlame,
            38 => Particle::Soul,
            39 => Particle::Flash,
            40 => Particle::HappyVillager,
            41 => Particle::Composter,
            42 => Particle::Heart,
            43 => Particle::InstantEffect,
            44 => Particle::Item(Decode::decode(r)?),
            45 => match <VarInt>::decode(r)? {
                VarInt(0) => Particle::VibrationBlock {
                    block_pos: BlockPos::decode(r)?,
                    ticks: VarInt::decode(r)?.0,
                },
                VarInt(1) => Particle::VibrationEntity {
                    entity_id: VarInt::decode(r)?.0,
                    entity_eye_height: f32::decode(r)?,
                    ticks: VarInt::decode(r)?.0,
                },
                invalid => bail!("invalid vibration position source of \"{}\"", invalid.0),
            },
            46 => Particle::ItemSlime,
            47 => Particle::ItemCobweb,
            48 => Particle::ItemSnowball,
            49 => Particle::LargeSmoke,
            50 => Particle::Lava,
            51 => Particle::Mycelium,
            52 => Particle::Note,
            53 => Particle::Poof,
            54 => Particle::Portal,
            55 => Particle::Rain,
            56 => Particle::Smoke,
            57 => Particle::WhiteSmoke,
            58 => Particle::Sneeze,
            59 => Particle::Spit,
            60 => Particle::SquidInk,
            61 => Particle::SweepAttack,
            62 => Particle::TotemOfUndying,
            63 => Particle::Underwater,
            64 => Particle::Splash,
            65 => Particle::Witch,
            66 => Particle::BubblePop,
            67 => Particle::CurrentDown,
            68 => Particle::BubbleColumnUp,
            69 => Particle::Nautilus,
            70 => Particle::Dolphin,
            71 => Particle::CampfireCosySmoke,
            72 => Particle::CampfireSignalSmoke,
            73 => Particle::DrippingHoney,
            74 => Particle::FallingHoney,
            75 => Particle::LandingHoney,
            76 => Particle::FallingNectar,
            77 => Particle::FallingSporeBlossom,
            78 => Particle::Ash,
            79 => Particle::CrimsonSpore,
            80 => Particle::WarpedSpore,
            81 => Particle::SporeBlossomAir,
            82 => Particle::DrippingObsidianTear,
            83 => Particle::FallingObsidianTear,
            84 => Particle::LandingObsidianTear,
            85 => Particle::ReversePortal,
            86 => Particle::WhiteAsh,
            87 => Particle::SmallFlame,
            88 => Particle::Snowflake,
            89 => Particle::DrippingDripstoneLava,
            90 => Particle::FallingDripstoneLava,
            91 => Particle::DrippingDripstoneWater,
            92 => Particle::FallingDripstoneWater,
            93 => Particle::GlowSquidInk,
            94 => Particle::Glow,
            95 => Particle::WaxOn,
            96 => Particle::WaxOff,
            97 => Particle::ElectricSpark,
            98 => Particle::Scrape,
            99 => Particle::Shriek {
                delay: VarInt::decode(r)?.0,
            },
            100 => Particle::EggCrack,
            101 => Particle::DustPlume,
            102 => Particle::TrialSpawnerDetection,
            103 => Particle::TrialSpawnerDetectionOminous,
            104 => Particle::VaultConnection,
            105 => Particle::DustPillar(BlockState::decode(r)?),
            106 => Particle::OminousSpawning,
            107 => Particle::RaidOmen,
            108 => Particle::TrialOmen,
            id => bail!("invalid particle ID of {id}"),
        })
    }
}

/// Encodes the particle without an ID.
impl Encode for Particle {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        match self {
            Particle::Block(block_state) => block_state.encode(w),
            Particle::BlockMarker(block_state) => block_state.encode(w),
            Particle::Dust { rgb, scale } => {
                rgb.encode(&mut w)?;
                scale.encode(w)
            }
            Particle::DustColorTransition {
                from_rgb,
                scale,
                to_rgb,
            } => {
                from_rgb.encode(&mut w)?;
                scale.encode(&mut w)?;
                to_rgb.encode(w)
            }
            Particle::FallingDust(block_state) => block_state.encode(w),
            Particle::SculkCharge { roll } => roll.encode(w),
            Particle::Item(stack) => stack.encode(w),
            Particle::VibrationBlock { block_pos, ticks } => {
                VarInt(0).encode(&mut w)?;
                block_pos.encode(&mut w)?;
                VarInt(*ticks).encode(w)
            }
            Particle::VibrationEntity {
                entity_id,
                entity_eye_height,
                ticks,
            } => {
                VarInt(1).encode(&mut w)?;
                VarInt(*entity_id).encode(&mut w)?;
                entity_eye_height.encode(&mut w)?;
                VarInt(*ticks).encode(w)
            }
            Particle::Shriek { delay } => VarInt(*delay).encode(w),
            Particle::EntityEffect { color } => color.encode(w),
            Particle::DustPillar(block_state) => block_state.encode(w),
            _ => Ok(()),
        }
    }
}