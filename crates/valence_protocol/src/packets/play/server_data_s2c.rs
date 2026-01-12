use std::borrow::Cow;

use valence_binary::{Decode, Encode, Packet, TextComponent};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct ServerDataS2c<'a> {
    pub motd: Cow<'a, TextComponent>,
    pub icon: Option<&'a [u8]>,
}
