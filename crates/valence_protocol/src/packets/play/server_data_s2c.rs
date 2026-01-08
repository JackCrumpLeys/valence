use std::borrow::Cow;

use crate::text_component::TextComponent;
use crate::{Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct ServerDataS2c<'a> {
    pub motd: Cow<'a, TextComponent>,
    pub icon: Option<&'a [u8]>,
}
