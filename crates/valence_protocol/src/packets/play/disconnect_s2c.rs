use std::borrow::Cow;

use crate::text_component::TextComponent;
use crate::{Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct DisconnectS2c<'a> {
    pub reason: Cow<'a, TextComponent>,
}
