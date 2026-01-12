use super::select_known_packs_s2c::KnownPack;
use crate::{Packet, PacketState};
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
pub struct SelectKnownPacksC2s<'a> {
    pub packs: Vec<KnownPack<'a>>,
}
