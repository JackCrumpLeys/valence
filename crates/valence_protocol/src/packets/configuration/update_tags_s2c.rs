use std::borrow::Cow;

use crate::packets::play::update_tags_s2c::RegistryMap;
use crate::{Packet, PacketState};
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
pub struct UpdateTagsS2c<'a> {
    pub groups: Cow<'a, RegistryMap>,
}
