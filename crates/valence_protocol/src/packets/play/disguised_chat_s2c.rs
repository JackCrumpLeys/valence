use std::borrow::Cow;

use crate::text_component::TextComponent;
use crate::{Decode, Encode, Packet, VarInt};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct DisguisedChatS2c<'a> {
    pub message: Cow<'a, TextComponent>,
    pub chat_type: VarInt,
    pub sender_name: Cow<'a, TextComponent>,
    pub target_name: Option<Cow<'a, TextComponent>>,
}
