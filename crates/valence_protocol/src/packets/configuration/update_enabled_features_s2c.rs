use std::borrow::Cow;

use valence_ident::Ident;

use valence_binary::{Decode, Encode, Packet, PacketState};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
pub struct UpdateEnabledFeaturesS2c<'a> {
    pub features: Vec<Ident<Cow<'a, str>>>,
}
