use bevy_ecs::prelude::*;
use valence_protocol_macros::{Decode, Encode};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, Component)]
pub enum Direction {
    /// -Y
    Down,
    /// +Y
    Up,
    /// -Z
    North,
    /// +Z
    South,
    /// -X
    West,
    /// +X
    East,
}
