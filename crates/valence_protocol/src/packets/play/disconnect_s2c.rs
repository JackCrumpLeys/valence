use std::borrow::Cow;

use valence_binary::{Decode, Encode, Packet, TextComponent};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct DisconnectS2c<'a> {
    pub reason: Cow<'a, TextComponent>,
}
