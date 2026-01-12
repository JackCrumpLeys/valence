use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct DebugSampleSubscriptionC2s {
    pub sample_type: DebugSampleType,
}
#[derive(Clone, Debug, Encode, Decode)]
pub enum DebugSampleType {
    TickTime,
}
