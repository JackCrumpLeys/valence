use std::borrow::Cow;

use crate::{Packet, PacketState};
use valence_binary::TextComponent;
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
/// Sent by the server to the client to disconnect them. The reason is displayed
/// to the client.
pub struct DisconnectS2c<'a> {
    pub reason: Cow<'a, TextComponent>,
}
