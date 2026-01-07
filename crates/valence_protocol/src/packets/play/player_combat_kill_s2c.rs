use std::borrow::Cow;

use valence_text::Text;

use crate::{text_component::TextComponent, Decode, Encode, Packet, VarInt};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct PlayerCombatKillS2c<'a> {
    pub player_id: VarInt,
    pub message: Cow<'a, TextComponent>,
}
