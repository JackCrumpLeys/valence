use std::borrow::Cow;

use crate::text_component::TextComponent;
use crate::{Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SetTitleTextS2c<'a> {
    pub title_text: Cow<'a, TextComponent>,
}
