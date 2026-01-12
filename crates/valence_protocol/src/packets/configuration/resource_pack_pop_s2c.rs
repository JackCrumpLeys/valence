use uuid::Uuid;

use crate::{Packet, PacketState};
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
/// If uuid is None, all resource packs are removed. Else, only the resource
/// pack with the given uuid is removed.
pub struct ResourcePackPopS2c(pub Option<Uuid>);
