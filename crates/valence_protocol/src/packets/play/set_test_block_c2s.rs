use std::borrow::Cow;

use crate::BlockPos;
use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SetTestBlockC2s<'a> {
    pub position: BlockPos,
    pub mode: SetTestBlockMode,
    pub message: Cow<'a, str>,
}

#[derive(Copy, Clone, Debug, Encode, Decode)]
pub enum SetTestBlockMode {
    Start,
    Log,
    Fail,
    Accept,
}
