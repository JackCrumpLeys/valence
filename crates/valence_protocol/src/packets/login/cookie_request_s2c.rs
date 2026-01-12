use std::borrow::Cow;

use valence_ident::Ident;

use crate::{Packet, PacketState};
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Login)]
/// Sent by the server to the client to request a cookie.
pub struct CookieRequestS2c<'a> {
    pub key: Ident<Cow<'a, str>>,
}
