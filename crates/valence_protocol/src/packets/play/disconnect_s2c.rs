use std::borrow::Cow;

use crate::Packet;
use valence_binary::{Decode, Encode, TextComponent};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct DisconnectS2c<'a> {
    pub reason: Cow<'a, TextComponent>,
}
