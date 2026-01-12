pub use valence_generated::sound::Sound;
use valence_ident::Ident;

use valence_binary::IdOr;
use valence_binary::{Decode, Encode};

pub type SoundId = IdOr<SoundDirect>;

#[derive(Clone, Debug, Encode, Decode, PartialEq)]
pub struct SoundDirect {
    pub id: Ident<String>,
    pub range: Option<f32>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode)]
pub enum SoundCategory {
    Master,
    Music,
    Record,
    Weather,
    Block,
    Hostile,
    Neutral,
    Player,
    Ambient,
    Voice,
}
