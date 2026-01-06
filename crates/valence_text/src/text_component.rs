use std::io::Write;

use anyhow::ensure;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use valence_nbt::binary::{FromModifiedUtf8, ToModifiedUtf8};
use valence_nbt::serde::ser::CompoundSerializer;
use valence_nbt::{Compound, Tag};
use valence_text::{IntoText, Text};

use crate::{Decode, Encode};

#[derive(Clone, Debug)]
pub enum TextComponent {
    Compound(Text),
    String(NbtStringText),
}

/// A wrapper around `Text` that encodes and decodes as an NBT String.
#[derive(Clone, Debug)]
pub struct NbtStringText(pub Text);

impl Encode for NbtStringText {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        let _ = w.write(&[Tag::String as u8])?;

        let string = self.0.to_legacy_lossy();
        let len = string.modified_uf8_len();

        match len.try_into() {
            Ok(n) => w.write_u16::<BigEndian>(n)?,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "string of length {len} exceeds maximum of u16::MAX"
                ))
            }
        }

        string.to_modified_utf8(len, &mut w)?;
        Ok(())
    }
}

impl Decode<'_> for NbtStringText {
    fn decode(r: &mut &'_ [u8]) -> anyhow::Result<Self> {
        let len = r.read_u16::<BigEndian>()?.into();
        ensure!(
            len <= r.len(),
            "string of length {} exceeds remainder of input {}",
            len,
            r.len()
        );

        let (left, right) = r.split_at(len);

        let string = match String::from_modified_utf8(left) {
            Ok(string) => {
                *r = right;
                string
            }
            Err(_) => return Err(anyhow::anyhow!("could not decode modified UTF-8 data")),
        };

        Ok(Self(string.into_text()))
    }
}

impl Encode for TextComponent {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        match self {
            TextComponent::Compound(text) => text.encode(&mut w),
            TextComponent::String(nbt_string_text) => nbt_string_text.encode(&mut w),
        }
    }
}

impl Decode<'_> for TextComponent {
    fn decode(r: &mut &'_ [u8]) -> anyhow::Result<Self> {
        let tag_id = dbg!(r.read_u8()?);

        match tag_id {
            x if x == Tag::String as u8 => {
                let nbt_string_text = NbtStringText::decode(r)?;
                Ok(TextComponent::String(nbt_string_text))
            }
            x if x == Tag::Compound as u8 => Ok(TextComponent::Compound(Decode::decode(r)?)),
            _ => Err(anyhow::anyhow!(
                "unexpected tag ID {tag_id} when decoding TextComponent"
            )),
        }
    }
}
