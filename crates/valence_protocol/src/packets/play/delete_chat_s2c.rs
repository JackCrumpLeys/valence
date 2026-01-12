use super::player_chat_s2c::MessageSignature;
use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct DeleteChatS2c<'a> {
    pub signature: MessageSignature<'a>,
}
