use std::borrow::Cow;

use valence_generated::block::BlockEntityKind;
use valence_nbt::Compound;

use crate::array::FixedArray;
use crate::{ChunkPos, Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct LevelChunkWithLightS2c<'a> {
    pub pos: ChunkPos,
    pub heightmaps: Cow<'a, [HeightMap]>,
    pub blocks_and_biomes: &'a [u8],
    pub block_entities: Cow<'a, [ChunkDataBlockEntity<'a>]>,
    pub sky_light_mask: Cow<'a, [u64]>,
    pub block_light_mask: Cow<'a, [u64]>,
    pub empty_sky_light_mask: Cow<'a, [u64]>,
    pub empty_block_light_mask: Cow<'a, [u64]>,
    pub sky_light_arrays: Cow<'a, [FixedArray<u8, 2048>]>,
    pub block_light_arrays: Cow<'a, [FixedArray<u8, 2048>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
// TODO: force every packet to always include all 3 heightmaps?
pub struct HeightMap {
    pub kind: HeightMapKind,
    pub data: Vec<i64>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Encode, Decode)]
pub enum HeightMapKind {
    /// All blocks other than air, cave air and void air.
    WorldSurface = 1,
    /// "Solid" blocks, except bamboo saplings and cactuses; fluids.
    MotionBlocking = 4,
    /// Same as `MOTION_BLOCKING`, excluding leaf blocks.
    MotionBlockingNoLeaves = 5,
}

// impl Encode for HeightMapKind {
//     fn encode(&self, mut w: impl std::io::Write) -> anyhow::Result<()> {
//         let kind = match self {
//             HeightMapKind::WorldSurface => 1,
//             HeightMapKind::MotionBlocking => 4,
//             HeightMapKind::MotionBlockingNoLeaves => 5,
//         };
//         VarInt(kind).encode(&mut w)
//     }
// }

// impl Decode<'_> for HeightMapKind {
//     fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
//         let kind = VarInt::decode(r)?;
//         match kind.0 {
//             1 => Ok(HeightMapKind::WorldSurface),
//             4 => Ok(HeightMapKind::MotionBlocking),
//             5 => Ok(HeightMapKind::MotionBlockingNoLeaves),
//             _ => Err(anyhow::anyhow!("unknown height map kind: {}", kind.0)),
//         }
//     }
// }

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct ChunkDataBlockEntity<'a> {
    pub packed_xz: i8,
    pub y: i16,
    pub kind: BlockEntityKind,
    pub data: Cow<'a, Compound>,
}
