use std::borrow::Cow;

use crate::Packet;
use crate::Ident;
use valence_binary::{Decode, Encode, VarInt};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct CooldownS2c<'a> {
    pub cooldown_group: Ident<Cow<'a, str>>,
    pub cooldown_ticks: VarInt,
}
