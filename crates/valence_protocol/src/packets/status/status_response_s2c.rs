use crate::{Packet, PacketState};
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Status)]
pub struct StatusResponseS2c<'a> {
    /// See <https://minecraft.wiki/w/Java_Edition_protocol/Server_List_Ping#Status_Response>.
    pub json: &'a str,
}
