use crate::registry_id::StaticRegistry;
use std::fmt::Debug;
use std::io::Write;

use crate::registry_id::{RegistryId, RegistryItem};
use anyhow::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Decode, Encode, VarInt};

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(untagged)]
#[serde(bound(deserialize = "R: RegistryItem + StaticRegistry, Inline: Deserialize<'de>"))]
pub enum IdOr<R: RegistryItem, Inline = R> {
    Id(RegistryId<R>),
    Inline(Inline),
}

impl<R: RegistryItem, Inline> From<RegistryId<R>> for IdOr<R, Inline> {
    fn from(id: RegistryId<R>) -> Self {
        Self::Id(id)
    }
}

impl<T: RegistryItem, U> IdOr<T, U> {
    pub fn id<I: Into<RegistryId<T>>>(id: I) -> Self {
        Self::Id(id.into())
    }

    pub fn inline(value: U) -> Self {
        Self::Inline(value)
    }
}

impl<T: RegistryItem, U: Encode> Encode for IdOr<T, U> {
    fn encode(&self, mut buf: impl Write) -> anyhow::Result<()> {
        match self {
            Self::Id(id) => (id.get() + 1).encode(buf),
            Self::Inline(value) => {
                VarInt(0).encode(&mut buf).unwrap();
                value.encode(&mut buf)
            }
        }
    }
}

impl<'a, T: RegistryItem, U: Decode<'a>> Decode<'a> for IdOr<T, U> {
    fn decode(buf: &mut &'a [u8]) -> Result<Self, Error> {
        let id = VarInt::decode(buf)?;
        if id == VarInt(0) {
            let value = U::decode(buf)?;
            Ok(Self::Inline(value))
        } else {
            let registry_id = RegistryId::new(id.0 - 1);
            Ok(Self::Id(registry_id))
        }
    }
}
