use uuid::Uuid;

use valence_binary::TextComponent;
use valence_binary::{Bounded, Decode, Encode, Packet, PacketState};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration, )]
pub struct ResourcePackPushS2c<'a> {
    pub uuid: Uuid,
    pub url: Bounded<&'a str, 32767>,
    pub hash: Bounded<&'a str, 40>,
    pub prompt_message: Option<TextComponent>,
}
