use std::borrow::Cow;

use valence_binary::{Decode, Encode};
use valence_ident::Ident;

use valence_binary::block_pos::BlockPos;

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode)]
pub struct GlobalPos<'a> {
    pub dimension_name: Ident<Cow<'a, str>>,
    pub position: BlockPos,
}
