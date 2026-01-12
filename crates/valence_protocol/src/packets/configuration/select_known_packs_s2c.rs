use std::borrow::Cow;

use crate::{Packet, PacketState};
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
pub struct SelectKnownPacksS2c<'a> {
    pub packs: Vec<KnownPack<'a>>,
}

#[derive(Clone, Debug, Encode, Decode)]
pub struct KnownPack<'a> {
    pub namespace: Cow<'a, str>,
    pub id: Cow<'a, str>,
    pub version: Cow<'a, str>,
}
