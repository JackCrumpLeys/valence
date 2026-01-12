use std::borrow::Cow;

use crate::Packet;
use valence_binary::{Decode, Encode, TextComponent};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SystemChatS2c<'a> {
    pub chat: Cow<'a, TextComponent>,
    /// Whether the message is in the actionbar or the chat.
    pub overlay: bool,
}
