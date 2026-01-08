use std::borrow::Cow;
use std::io::Write;

use anyhow::ensure;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use valence_nbt::Tag;
use valence_text::{IntoText, Text};

use crate::{Decode, Encode};

#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)] // if you change this you have to remove the unsafe code!
pub struct TextComponent {
    pub text: Text,
}

impl TextComponent {
    /// Zero-copy cast from a Cow<Text> to a Cow<TextComponent>.
    ///
    /// # Safety
    /// This is safe because `TextComponent` is #[repr(transparent)] wrapper
    /// around Text.
    pub fn from_cow_text<'a>(cow: Cow<'a, Text>) -> Cow<'a, TextComponent> {
        match cow {
            Cow::Borrowed(b) => {
                // SAFETY: TextComponent has the exact same memory layout as Text.
                let ptr = b as *const Text as *const TextComponent;
                Cow::Borrowed(unsafe { &*ptr })
            }
            Cow::Owned(o) => Cow::Owned(TextComponent { text: o }),
        }
    }

    pub fn as_text(&self) -> &Text {
        &self.text
    }
}

impl<'a> IntoText<'a> for TextComponent {
    fn into_cow_text(self) -> Cow<'a, Text> {
        // Since we wrap Text, we just return it.
        Cow::Owned(self.text)
    }
}

pub trait IntoTextComponent<'a> {
    fn into_text_component(self) -> TextComponent;
    fn into_cow_text_component(self) -> Cow<'a, TextComponent>;
}

impl<'a, T: IntoText<'a>> IntoTextComponent<'a> for T {
    fn into_text_component(self) -> TextComponent {
        TextComponent {
            text: self.into_cow_text().into_owned(),
        }
    }

    fn into_cow_text_component(self) -> Cow<'a, TextComponent> {
        let cow = self.into_cow_text();
        TextComponent::from_cow_text(cow)
    }
}

impl From<Text> for TextComponent {
    fn from(value: Text) -> Self {
        TextComponent { text: value }
    }
}

/// A wrapper around `Text` that encodes and decodes as an NBT String.
#[derive(Clone, Debug, PartialEq)]
pub struct NbtStringText(pub Text);

impl Encode for NbtStringText {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        w.write_u8(Tag::String as u8)?;

        let string = self.0.to_legacy_lossy();
        // Assuming modified_utf8 logic is on the string type
        let len = string.len(); // Simplified for snippet context

        match u16::try_from(len) {
            Ok(n) => w.write_u16::<BigEndian>(n)?,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "string of length {len} exceeds maximum of u16::MAX"
                ));
            }
        }

        // Write string bytes... (placeholder for `to_modified_utf8`)
        w.write_all(string.as_bytes())?;
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

        // Placeholder for from_modified_utf8
        let string_val = String::from_utf8_lossy(left).into_owned();
        *r = right;

        // Assuming String can turn into Text
        Ok(Self(Text::from(string_val)))
    }
}

impl Encode for TextComponent {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        // Logic moved here: Check plainness to decide format
        if self.text.is_plain() {
            // Encode as NBT String
            NbtStringText(self.text.clone()).encode(w)
        } else {
            // Encode as Compound
            w.write_u8(Tag::Compound as u8)?;
            self.text.encode(&mut w)
        }
    }
}

impl Decode<'_> for TextComponent {
    fn decode(r: &mut &'_ [u8]) -> anyhow::Result<Self> {
        let tag_id = r.read_u8()?;
        match tag_id {
            val if val == Tag::String as u8 => {
                // Decode specific NBT String logic
                let nbt_string_text = NbtStringText::decode(r)?;
                Ok(TextComponent {
                    text: nbt_string_text.0,
                })
            }
            val if val == Tag::Compound as u8 => {
                // Standard Text decode
                Ok(TextComponent {
                    text: Decode::decode(r)?,
                })
            }
            _ => Err(anyhow::anyhow!(
                "unexpected tag ID {tag_id} when decoding TextComponent"
            )),
        }
    }
}
