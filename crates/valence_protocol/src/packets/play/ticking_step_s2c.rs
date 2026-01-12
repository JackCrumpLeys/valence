use valence_binary::{Decode, Encode, Packet, VarInt};
#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct TickingStepS2c {
    pub tick_steps: VarInt,
}
