use std::borrow::Cow;

use crate::Packet;
use valence_binary::{Decode, Encode, TextComponent};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct ServerDataS2c<'a> {
    pub motd: Cow<'a, TextComponent>,
    pub icon: Option<&'a [u8]>,
}
