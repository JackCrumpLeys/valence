use std::borrow::Cow;

use valence_ident::Ident;

use valence_binary::{Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
/// Stores a cookie on the client
pub struct StoreCookieS2c<'a> {
    pub key: Ident<Cow<'a, str>>,
    pub payload: Cow<'a, [u8]>,
}
