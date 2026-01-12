use crate::Packet;
use valence_binary::{Decode, Encode};

/// Unused by notchian clients.
#[derive(Copy, Clone, PartialEq, Debug, Encode, Decode, Packet)]
pub struct PlayerCombatEnterS2c;
