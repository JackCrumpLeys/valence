use std::borrow::Cow;

use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct ChatCommandC2s<'a> {
    pub command: Cow<'a, str>,
}
