use valence_server::{protocol::{anyhow, packets::play::{container_click_c2s::SlotChange, ContainerClickC2s}}, ItemStack};

use crate::{CursorItem, Inventory};

/// This function simulates the "item click" action on the server 
/// and validates it.
/// If the action is valid: `Ok`,
/// We return the updated cursor item and the slot changes.
/// 
/// We need to compute those values in the validation because the packet no longer 
/// contains this data (item stacks are hashed now). 
pub(super) fn validate_click_slot_packet(
    packet: &ContainerClickC2s,
    player_inventory: &Inventory,
    open_inventory: Option<&Inventory>,
    cursor_item: &CursorItem,
) -> anyhow::Result<(ItemStack, Vec<SlotChange>)> {
    todo!()
}