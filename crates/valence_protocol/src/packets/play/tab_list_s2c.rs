use std::borrow::Cow;

use crate::text_component::TextComponent;
use crate::{Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct TabListS2c<'a> {
    pub header: Cow<'a, TextComponent>,
    pub footer: Cow<'a, TextComponent>,
}
