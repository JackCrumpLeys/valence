use std::hash::Hasher;

use crc32c::Crc32cHasher;

/// Hash a value using CRC32C as defined in minecraft's `HashOps.java` file.
pub trait HashOpsHashable {
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

mod tests {
    use crate::{item::ItemComponent, Encode, VarInt};

    use super::*;

    
    #[test]
    fn test_item_component_hashing() {
        let mut data = Vec::new();

        let comp = ItemComponent::Unbreakable;
        let hash = comp.hash();
        
        assert_eq!(hash, -982207288);

        let comp = ItemComponent::MaxStackSize { max_stack_size: VarInt(10) };
        let hash = comp.hash();
        assert_eq!(hash, -919192125);
    }
}