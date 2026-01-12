use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub enum ClientCommandC2s {
    PerformRespawn,
    RequestStats,
}
