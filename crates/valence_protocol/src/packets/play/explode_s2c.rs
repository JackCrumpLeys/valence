use valence_math::DVec3;

use crate::Packet;
use crate::{sound::SoundId, Particle};
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct ExplodeS2c {
    pub pos: DVec3,
    pub player_motion: Option<DVec3>,
    pub particle: Particle,
    pub sound: SoundId,
}
