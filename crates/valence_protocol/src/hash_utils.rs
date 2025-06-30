use std::hash::Hasher;

use crc32c::Crc32cHasher;

/// Hash a value using CRC32C as defined in minecraft's `HashOps.java` file.
pub(crate) trait HashOpsHashable {
    fn hash(&self) -> u32;
}

impl HashOpsHashable for i8 {
    fn hash(&self) -> u32 {
        let mut hasher = Crc32cHasher::default();
        hasher.write_u8(6);
        hasher.write_i8(*self);
        hasher.finish() as u32
    }
}

impl HashOpsHashable for i16 {
    fn hash(&self) -> u32 {
        let mut hasher = Crc32cHasher::default();
        hasher.write_u8(7);
        hasher.write_i16(*self);
        hasher.finish() as u32
    }
}

impl HashOpsHashable for i32 {
    fn hash(&self) -> u32 {
        let mut hasher = Crc32cHasher::default();
        hasher.write_u8(8);
        hasher.write_i32(*self);
        hasher.finish() as u32
    }
}

impl HashOpsHashable for i64 {
    fn hash(&self) -> u32 {
        let mut hasher = Crc32cHasher::default();
        hasher.write_u8(9);
        hasher.write_i64(*self);
        hasher.finish() as u32
    }
}

impl HashOpsHashable for f32 {
    fn hash(&self) -> u32 {
        let mut hasher = Crc32cHasher::default();
        hasher.write_u8(10);
        hasher.write(&self.to_le_bytes());
        hasher.finish() as u32
    }
}

impl HashOpsHashable for f64 {
    fn hash(&self) -> u32 {
        let mut hasher = Crc32cHasher::default();
        hasher.write_u8(11);
        hasher.write(&self.to_le_bytes());
        hasher.finish() as u32
    }
}

impl HashOpsHashable for String {
    fn hash(&self) -> u32 {
        let mut hasher = Crc32cHasher::default();
        hasher.write_u8(12);
        hasher.write(&(self.chars().count() as i32).to_le_bytes());
        for c in self.chars() {
            let c = c as i16;
            hasher.write_i8(c as i8);
            hasher.write_i8((c >> 8) as i8);
        }

        hasher.finish() as u32
    }
}

impl HashOpsHashable for bool {
    fn hash(&self) -> u32 {
        let mut hasher = Crc32cHasher::default();
        hasher.write_u8(13);
        hasher.write_u8(if *self { 1 } else { 0 });
        hasher.finish() as u32
    }
}

impl HashOpsHashable for Vec<u8> {
    fn hash(&self) -> u32 {
        let mut hasher = Crc32cHasher::default();
        hasher.write_u8(14);
        for &byte in self {
            hasher.write_u8(byte);
        }
        hasher.write_u8(15);
        hasher.finish() as u32
    }
}

impl HashOpsHashable for Vec<i32> {
    fn hash(&self) -> u32 {
        let mut hasher = Crc32cHasher::default();
        hasher.write_u8(16);
        for &value in self {
            hasher.write_i32(value);
        }
        hasher.write_u8(17);
        hasher.finish() as u32
    }
}

impl HashOpsHashable for Vec<i64> {
    fn hash(&self) -> u32 {
        let mut hasher = Crc32cHasher::default();
        hasher.write_u8(18);
        for &value in self {
            hasher.write_i64(value);
        }
        hasher.write_u8(19);
        hasher.finish() as u32
    }
}

// mod tests {

//     #[test]
//     fn test_item_component_hashing() {
//         let comp = ItemComponent::MaxStackSize { max_stack_size: VarInt(10)
// };         let expected_hash = -919192125;
//         assert_eq!(comp.hash(), expected_hash);

//         let comp = ItemComponent::Damage { damage: VarInt(25) };
//         let expected_hash = 1114637767;
//         assert_eq!(comp.hash(), expected_hash);

//         let comp = ItemComponent::Unbreakable;
//         let expected_hash = -982207288;
//         assert_eq!(comp.hash(), expected_hash);

//         let comp = ItemComponent::CustomName { name: "Custom
// Item".to_string().into() };         let expected_hash = -2064782911;
//         assert_eq!(comp.hash(), expected_hash);

//         let comp = ItemComponent::ItemName { name: "Item
// name".to_string().into() };         let expceted_hash = 789562212;
//         assert_eq!(comp.hash(), expceted_hash);

//         let comp = ItemComponent::ItemModel { model: ident!("model").into()
// };         let expected_hash = 1591847691;
//         assert_eq!(comp.hash(), expected_hash);

//         let comp = ItemComponent::Lore {
//             lines: vec![
//                 "Lore line 1".into(),
//                 "Lore line 2".into(),
//             ],
//         };
//         let expected_hash = 74152878;
//         assert_eq!(comp.hash(), expected_hash);

//         let comp = ItemComponent::Rarity { rarity: Rarity::Epic };
//         let expected_hash = -292715907;
//         assert_eq!(comp.hash(), expected_hash);

//         let comp = ItemComponent::Enchantments {
//             enchantments: vec![
//                 (VarInt(16), VarInt(1)), // Sharpness I
//                 (VarInt(19), VarInt(2)), // Knockback II
//             ]
//         };
//         let expected_hash = 1508412171;
//         assert_eq!(comp.hash(), expected_hash);

//         let comp = ItemComponent::TooltipDisplay {
//             hide_tooltip: false,
//             hidden_components: vec![
//                 VarInt(12),
//                 VarInt(13),
//             ],
//         };
//         let expected_hash = 1684687611;
//         assert_eq!(comp.hash(), expected_hash);

//         let comp = ItemComponent::RepairCost { cost: VarInt(5) };
//         let expected_hash = 645064431;
//         assert_eq!(comp.hash(), expected_hash);

//         let comp = ItemComponent::EnchantmentGlintOverride { has_glint: false
// };         let expected_hash = 828198337;
//         assert_eq!(comp.hash(), expected_hash);

//         let comp = ItemComponent::Food {
//             nutrition: VarInt(3),
//             saturation_modifier: 10.5,
//             can_always_eat: true,
//         };
//         let expected_hash = 1668104010;
//         assert_eq!(comp.hash(), expected_hash);

//     }
// }
