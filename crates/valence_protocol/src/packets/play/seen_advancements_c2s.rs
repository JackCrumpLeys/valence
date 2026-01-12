use std::borrow::Cow;

use valence_ident::Ident;

use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub enum SeenAdvancementsC2s<'a> {
    OpenedTab { tab_id: Ident<Cow<'a, str>> },
    ClosedScreen,
}
