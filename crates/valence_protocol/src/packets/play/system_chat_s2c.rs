use std::borrow::Cow;

use valence_text::Text;

use crate::text_component::TextComponent;
use crate::{Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SystemChatS2c<'a> {
    pub chat: Cow<'a, TextComponent>,
    /// Whether the message is in the actionbar or the chat.
    pub overlay: bool,
}
