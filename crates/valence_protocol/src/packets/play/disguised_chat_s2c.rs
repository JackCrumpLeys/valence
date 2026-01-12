use std::borrow::Cow;

use crate::Packet;
use valence_binary::{Decode, Encode, TextComponent, VarInt};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct DisguisedChatS2c<'a> {
    pub message: Cow<'a, TextComponent>,
    pub chat_type: VarInt,
    pub sender_name: Cow<'a, TextComponent>,
    pub target_name: Option<Cow<'a, TextComponent>>,
}
