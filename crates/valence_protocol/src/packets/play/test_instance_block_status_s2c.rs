use valence_math::DVec3;

use crate::text_component::TextComponent;
use crate::{Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct TestInstanceBlockStatusS2c {
    pub status: TextComponent,
    pub size: Option<DVec3>,
}
