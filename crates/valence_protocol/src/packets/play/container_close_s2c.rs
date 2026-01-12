use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct ContainerCloseS2c {
    /// Ignored by notchian clients.
    pub window_id: VarInt,
}
