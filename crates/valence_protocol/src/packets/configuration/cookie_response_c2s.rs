use std::borrow::Cow;

use valence_binary::{Decode, Encode};
use valence_ident::Ident;

use crate::{Packet, PacketState};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
/// Response to a cookie request from the server.
pub struct CookieResponseC2s<'a> {
    pub key: Ident<Cow<'a, str>>,
    pub payload: Option<Cow<'a, [u8]>>,
}
