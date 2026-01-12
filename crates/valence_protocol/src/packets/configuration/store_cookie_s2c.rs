use std::borrow::Cow;

use valence_ident::Ident;

use crate::{Packet, PacketState};
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
/// Stores a cookie on the client
pub struct StoreCookieS2c<'a> {
    pub key: Ident<Cow<'a, str>>,
    pub payload: Cow<'a, [u8]>,
}
