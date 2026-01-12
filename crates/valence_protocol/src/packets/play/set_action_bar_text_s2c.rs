use std::borrow::Cow;

use crate::Packet;
use valence_binary::{Decode, Encode, TextComponent};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SetActionBarTextS2c<'a> {
    pub action_bar_text: Cow<'a, TextComponent>,
}
