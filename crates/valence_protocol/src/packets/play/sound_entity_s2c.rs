use crate::sound::{SoundCategory, SoundId};
use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};
#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SoundEntityS2c {
    pub id: SoundId,
    pub category: SoundCategory,
    pub entity_id: VarInt,
    pub volume: f32,
    pub pitch: f32,
    pub seed: i64,
}
