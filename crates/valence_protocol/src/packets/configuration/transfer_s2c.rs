use std::borrow::Cow;

use valence_binary::{Decode, Encode, VarInt};
use valence_ident::Ident;

use crate::{Packet, PacketState};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
pub struct TransferS2c<'a> {
    pub host: Ident<Cow<'a, str>>,
    pub port: VarInt,
}
