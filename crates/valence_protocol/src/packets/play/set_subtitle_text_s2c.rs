use std::borrow::Cow;

use crate::text_component::TextComponent;
use crate::{Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SetSubtitleTextS2c<'a> {
    pub subtitle_text: Cow<'a, TextComponent>,
}
