use valence_server::{protocol::{anyhow::{self, ensure}, packets::play::{container_click_c2s::{ClickMode, SlotChange}, ContainerClickC2s}, VarInt}, ItemStack};

use crate::{player_inventory::PlayerInventory, CursorItem, Inventory, InventoryWindow};
use crate::validate::anyhow::bail;
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
    ensure!(
            (packet.window_id == VarInt(0)) == open_inventory.is_none(),
            "window id and open inventory mismatch: window_id: {} open_inventory: {}",
            packet.window_id.0,
            open_inventory.is_some()
        );

    let mut new_cursor_stack = cursor_item.0.clone();
    let mut new_slot_changes = Vec::with_capacity(packet.slot_changes.len());

    let max_slot = if let Some(open_inv) = open_inventory {
        // when the window is split, we can only access the main slots of player's
        // inventory
        PlayerInventory::MAIN_SIZE + open_inv.slot_count()
    } else {
        player_inventory.slot_count()
    };

    // check all slot ids and item counts are valid
    ensure!(
        packet.slot_changes.iter().all(|s| {
            if !(0..=max_slot).contains(&(s.idx as u16)) {
                return false;
            }

            if !s.stack.is_empty() {
                let max_stack_size = s.stack.item.max_stack().max(s.stack.count);
                if !(1..=max_stack_size).contains(&(s.stack.count)) {
                    return false;
                }
            }

            true
        }),
        "invalid slot ids or item counts"
    );

    // check carried item count is valid
    if !packet.carried_item.is_empty() {
        let carried_item = &packet.carried_item;

        let max_stack_size = carried_item.item.max_stack().max(carried_item.count);
        ensure!(
            (1..=max_stack_size).contains(&carried_item.count),
            "invalid carried item count"
        );
    }

    match packet.mode {
        ClickMode::Click => {
            ensure!((0..=1).contains(&packet.button), "invalid button");
            ensure!(
                (0..=max_slot).contains(&(packet.slot_idx as u16))
                    || packet.slot_idx == -999
                    || packet.slot_idx == -1,
                "invalid slot index"
            )
        }
        ClickMode::ShiftClick => {
            ensure!((0..=1).contains(&packet.button), "invalid button");
            ensure!(
                packet.carried_item.is_empty(),
                "carried item must be empty for a hotbar swap"
            );
            ensure!(
                (0..=max_slot).contains(&(packet.slot_idx as u16)),
                "invalid slot index"
            )
        }
        ClickMode::Hotbar => {
            ensure!(matches!(packet.button, 0..=8 | 40), "invalid button");
            ensure!(
                packet.carried_item.is_empty(),
                "carried item must be empty for a hotbar swap"
            );
        }
        ClickMode::CreativeMiddleClick => {
            ensure!(packet.button == 2, "invalid button");
            ensure!(
                (0..=max_slot).contains(&(packet.slot_idx as u16)),
                "invalid slot index"
            )
        }
        ClickMode::DropKey => {
            ensure!((0..=1).contains(&packet.button), "invalid button");
            ensure!(
                packet.carried_item.is_empty(),
                "carried item must be empty for an item drop"
            );
            ensure!(
                (0..=max_slot).contains(&(packet.slot_idx as u16)) || packet.slot_idx == -999,
                "invalid slot index"
            )
        }
        ClickMode::Drag => {
            ensure!(
                matches!(packet.button, 0..=2 | 4..=6 | 8..=10),
                "invalid button"
            );
            ensure!(
                (0..=max_slot).contains(&(packet.slot_idx as u16)) || packet.slot_idx == -999,
                "invalid slot index"
            )
        }
        ClickMode::DoubleClick => ensure!(packet.button == 0, "invalid button"),
    }

    // Check that items aren't being duplicated, i.e. conservation of mass.

    let window = InventoryWindow {
        player_inventory,
        open_inventory,
    };

    match packet.mode {
        ClickMode::Click => {
            if packet.slot_idx == -1 {
                // Clicked outside the allowed window
                ensure!(
                    packet.slot_changes.is_empty(),
                    "slot modifications must be empty"
                );

                let count_deltas = calculate_net_item_delta(packet, &window, cursor_item);
                ensure!(
                    count_deltas == 0,
                    "invalid item delta: expected 0, got {}",
                    count_deltas
                );
            } else if packet.slot_idx == -999 {
                // Clicked outside the window, so the client is dropping an item
                ensure!(
                    packet.slot_changes.is_empty(),
                    "slot modifications must be empty"
                );

                // Clicked outside the window
                let count_deltas = calculate_net_item_delta(packet, &window, cursor_item);
                let expected_delta = match packet.button {
                    1 => -1,
                    0 => {
                        if !cursor_item.is_empty() {
                            -i32::from(cursor_item.0.count)
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                };
                ensure!(
                    count_deltas == expected_delta,
                    "invalid item delta: expected {}, got {}",
                    expected_delta,
                    count_deltas
                );
            } else {
                // If the user clicked on an empty slot for example
                if packet.slot_changes.is_empty() {
                    let count_deltas = calculate_net_item_delta(packet, &window, cursor_item);
                    ensure!(
                        count_deltas == 0,
                        "invalid item delta: expected 0, got {}",
                        count_deltas
                    );
                } else {
                    ensure!(
                        packet.slot_changes.len() == 1,
                        "click must modify one slot, got {}",
                        packet.slot_changes.len()
                    );

                    let old_slot = window.slot(packet.slot_changes[0].idx as u16);
                    // TODO: make sure NBT is the same.
                    //       Sometimes, the client will add nbt data to an item if it's missing,
                    // like       "Damage" to a sword.
                    let should_swap: bool = packet.button == 0
                        && match (!old_slot.is_empty(), !cursor_item.is_empty()) {
                            (true, true) => old_slot.item != cursor_item.item,
                            (true, false) => true,
                            (false, true) => cursor_item.count <= cursor_item.item.max_stack(),
                            (false, false) => false,
                        };

                    if should_swap {
                        // assert that a swap occurs
                        ensure!(
                            // There are some cases where the client will add NBT data that
                            // did not previously exist.
                            old_slot.item == packet.carried_item.item
                                && old_slot.count == packet.carried_item.count
                                && cursor_item.0.item == packet.slot_changes[0].stack.item
                                && cursor_item.0.count == packet.slot_changes[0].stack.count,
                            "swapped items must match"
                        );
                    } else {
                        // assert that a merge occurs
                        let count_deltas = calculate_net_item_delta(packet, &window, cursor_item);
                        ensure!(
                            count_deltas == 0,
                            "invalid item delta for stack merge: {}",
                            count_deltas
                        );
                    }
                }
            }
        }
        ClickMode::ShiftClick => {
            // If the user clicked on an empty slot for example
            if packet.slot_changes.is_empty() {
                let count_deltas = calculate_net_item_delta(packet, &window, cursor_item);
                ensure!(
                    count_deltas == 0,
                    "invalid item delta: expected 0, got {}",
                    count_deltas
                );
            } else {
                ensure!(
                    (2..=3).contains(&packet.slot_changes.len()),
                    "shift click must modify 2 or 3 slots, got {}",
                    packet.slot_changes.len()
                );

                let count_deltas = calculate_net_item_delta(packet, &window, cursor_item);
                ensure!(
                    count_deltas == 0,
                    "invalid item delta: expected 0, got {}",
                    count_deltas
                );

                let Some(item_kind) = packet
                    .slot_changes
                    .iter()
                    .find(|s| !s.stack.is_empty())
                    .map(|s| s.stack.item)
                else {
                    bail!("shift click must move an item");
                };

                let old_slot_kind = window.slot(packet.slot_idx as u16).item;
                ensure!(
                    old_slot_kind == item_kind,
                    "shift click must move the same item kind as modified slots"
                );

                // assert all moved items are the same kind
                ensure!(
                    packet
                        .slot_changes
                        .iter()
                        .filter(|s| !s.stack.is_empty())
                        .all(|s| s.stack.item == item_kind),
                    "shift click must move the same item kind"
                );
            }
        }

        ClickMode::Hotbar => {
            if packet.slot_changes.is_empty() {
                let count_deltas = calculate_net_item_delta(packet, &window, cursor_item);
                ensure!(
                    count_deltas == 0,
                    "invalid item delta: expected 0, got {}",
                    count_deltas
                );
            } else {
                ensure!(
                    packet.slot_changes.len() == 2,
                    "hotbar swap must modify two slots, got {}",
                    packet.slot_changes.len()
                );

                let count_deltas = calculate_net_item_delta(packet, &window, cursor_item);
                ensure!(
                    count_deltas == 0,
                    "invalid item delta: expected 0, got {}",
                    count_deltas
                );

                // assert that a swap occurs
                let old_slots = [
                    window.slot(packet.slot_changes[0].idx as u16),
                    window.slot(packet.slot_changes[1].idx as u16),
                ];
                // There are some cases where the client will add NBT data that did not
                // previously exist.
                ensure!(
                    old_slots
                        .iter()
                        .any(|s| s.item == packet.slot_changes[0].stack.item
                            && s.count == packet.slot_changes[0].stack.count)
                        && old_slots
                            .iter()
                            .any(|s| s.item == packet.slot_changes[1].stack.item
                                && s.count == packet.slot_changes[1].stack.count),
                    "swapped items must match"
                );
            }
        }
        ClickMode::CreativeMiddleClick => {}
        ClickMode::DropKey => {
            if packet.slot_changes.is_empty() {
                let count_deltas = calculate_net_item_delta(packet, &window, cursor_item);
                ensure!(
                    count_deltas == 0,
                    "invalid item delta: expected 0, got {}",
                    count_deltas
                );
            } else {
                ensure!(
                    packet.slot_changes.len() == 1,
                    "drop key must modify exactly one slot"
                );
                ensure!(
                    packet.slot_idx == packet.slot_changes.first().map_or(-2, |s| s.idx),
                    "slot index does not match modified slot"
                );

                let old_slot = window.slot(packet.slot_idx as u16);
                let new_slot = &packet.slot_changes[0].stack;
                let is_transmuting = match (!old_slot.is_empty(), !new_slot.is_empty()) {
                    // TODO: make sure NBT is the same.
                    // Sometimes, the client will add nbt data to an item if it's missing, like
                    // "Damage" to a sword.
                    (true, true) => old_slot.item != new_slot.item,
                    (_, false) => false,
                    (false, true) => true,
                };
                ensure!(!is_transmuting, "transmuting items is not allowed");

                let count_deltas = calculate_net_item_delta(packet, &window, cursor_item);

                let expected_delta = match packet.button {
                    0 => -1,
                    1 => {
                        if !old_slot.is_empty() {
                            -i32::from(old_slot.count)
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                };
                ensure!(
                    count_deltas == expected_delta,
                    "invalid item delta: expected {}, got {}",
                    expected_delta,
                    count_deltas
                );
            }
        }
        ClickMode::Drag => {
            if matches!(packet.button, 2 | 6 | 10) {
                let count_deltas = calculate_net_item_delta(packet, &window, cursor_item);
                ensure!(
                    count_deltas == 0,
                    "invalid item delta: expected 0, got {}",
                    count_deltas
                );
            } else {
                ensure!(packet.slot_changes.is_empty() 
                    && packet.carried_item.item == cursor_item.0.item 
                    && packet.carried_item.count == cursor_item.0.count, 
                    "invalid drag state"
                );
            }
        }
        ClickMode::DoubleClick => {
            let count_deltas = calculate_net_item_delta(packet, &window, cursor_item);
            ensure!(
                count_deltas == 0,
                "invalid item delta: expected 0, got {}",
                count_deltas
            );
        }
    }

    // Preserve NBT data

    // Here we want to change the `new_slot`'s + `new_cursor_stack` based on the 
    // hashed slots in the original packet

    match packet.mode {
        ClickMode::Click => {

        },
        ClickMode::ShiftClick => {

        },
        ClickMode::Hotbar => {

        },
        ClickMode::CreativeMiddleClick => {

        },
        ClickMode::DropKey => {

        },
        ClickMode::Drag => {

        },
        ClickMode::DoubleClick => {

        },
    }

    Ok(())
}


/// Calculate the total difference in item counts if the changes in this packet
/// were to be applied.
///
/// Returns a positive number if items were added to the window, and a negative
/// number if items were removed from the window.
fn calculate_net_item_delta(
    packet: &ContainerClickC2s,
    window: &InventoryWindow,
    cursor_item: &CursorItem,
) -> i32 {
    let mut net_item_delta: i32 = 0;

    for slot in packet.slot_changes.iter() {
        let old_slot = window.slot(slot.idx as u16);
        let new_slot = &slot.stack;

        net_item_delta += match (!old_slot.is_empty(), !new_slot.is_empty()) {
            (true, true) => i32::from(new_slot.count) - i32::from(old_slot.count),
            (true, false) => -i32::from(old_slot.count),
            (false, true) => i32::from(new_slot.count),
            (false, false) => 0,
        };
    }

    net_item_delta += match (!cursor_item.is_empty(), !packet.carried_item.is_empty()) {
        (true, true) => i32::from(packet.carried_item.count) - i32::from(cursor_item.count),
        (true, false) => -i32::from(cursor_item.count),
        (false, true) => i32::from(packet.carried_item.count),
        (false, false) => 0,
    };

    net_item_delta
}