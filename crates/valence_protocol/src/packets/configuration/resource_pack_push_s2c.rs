use uuid::Uuid;

use crate::{Packet, PacketState};
use valence_binary::TextComponent;
use valence_binary::{Bounded, Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration, )]
pub struct ResourcePackPushS2c<'a> {
    pub uuid: Uuid,
    pub url: Bounded<&'a str, 32767>,
    pub hash: Bounded<&'a str, 40>,
    pub prompt_message: Option<TextComponent>,
}
