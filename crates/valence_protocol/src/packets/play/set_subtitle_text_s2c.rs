use std::borrow::Cow;

use crate::Packet;
use valence_binary::{Decode, Encode, TextComponent};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SetSubtitleTextS2c<'a> {
    pub subtitle_text: Cow<'a, TextComponent>,
}
