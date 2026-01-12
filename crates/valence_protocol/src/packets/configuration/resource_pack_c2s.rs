use uuid::Uuid;

use crate::packets::play::resource_pack_c2s::ResourcePackStatus;
use crate::{Packet, PacketState};
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
pub struct ResourcePackC2s {
    uuid: Uuid,
    result: ResourcePackStatus,
}
