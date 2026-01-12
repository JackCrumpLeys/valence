use std::borrow::Cow;

use uuid::Uuid;

use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode, Packet)]
pub struct PlayerInfoRemoveS2c<'a> {
    pub uuids: Cow<'a, [Uuid]>,
}
