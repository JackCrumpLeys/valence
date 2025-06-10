use std::borrow::Cow;

use crate::{BlockPos, Decode, Encode, Packet};

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
