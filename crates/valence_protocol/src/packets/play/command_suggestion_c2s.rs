use crate::Packet;
use valence_binary::{Bounded, Decode, Encode, VarInt};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct CommandSuggestionC2s<'a> {
    pub transaction_id: VarInt,
    pub text: Bounded<&'a str, 32500>,
}
