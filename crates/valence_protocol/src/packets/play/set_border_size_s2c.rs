use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SetBorderSizeS2c {
    pub diameter: f64,
}
