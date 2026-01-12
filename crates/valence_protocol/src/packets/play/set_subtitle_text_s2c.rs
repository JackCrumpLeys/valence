use std::borrow::Cow;

use valence_binary::{Decode, Encode, Packet, TextComponent};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SetSubtitleTextS2c<'a> {
    pub subtitle_text: Cow<'a, TextComponent>,
}
