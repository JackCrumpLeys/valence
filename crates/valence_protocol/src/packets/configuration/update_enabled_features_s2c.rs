use std::borrow::Cow;

use valence_ident::Ident;

use crate::{Packet, PacketState};
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
pub struct UpdateEnabledFeaturesS2c<'a> {
    pub features: Vec<Ident<Cow<'a, str>>>,
}
