use crate::{Packet, PacketState};
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
/// Sent by the server to the client to reset the chat.
pub struct ResetChatS2c;
