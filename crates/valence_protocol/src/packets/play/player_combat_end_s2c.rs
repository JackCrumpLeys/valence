use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};

/// Unused by notchian clients.
#[derive(Copy, Clone, PartialEq, Debug, Encode, Decode, Packet)]
pub struct PlayerCombatEndS2c {
    pub duration: VarInt,
}
