use crate::Packet;
use valence_binary::{Decode, Encode, VarInt};
#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SetSimulationDistanceS2c {
    pub simulation_distance: VarInt,
}
