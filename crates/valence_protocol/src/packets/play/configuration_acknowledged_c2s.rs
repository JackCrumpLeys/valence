use valence_binary::{Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct ConfigurationAcknowledgedC2s;
