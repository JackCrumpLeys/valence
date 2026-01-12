use std::borrow::Cow;

use valence_binary::{Decode, Encode, Packet, TextComponent, VarInt};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct PlayerCombatKillS2c<'a> {
    pub player_id: VarInt,
    pub message: Cow<'a, TextComponent>,
}
