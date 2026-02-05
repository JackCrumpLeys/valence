use std::borrow::Cow;

use valence_binary::registry_id::{RegistryId, RegistryItem, StaticRegistry};
use valence_protocol::{ident, Ident};

use crate::EntityKind;

impl RegistryItem for EntityKind {
    const KEY: Ident<&'static str> = ident!("minecraft:entity_type");
}

impl StaticRegistry for EntityKind {
    fn from_registry_id(id: RegistryId<Self>) -> Option<Self> {
        EntityKind(id.get())
    }

    fn to_registry_id(self) -> RegistryId<Self> {
        RegistryId::new(self.0)
    }

    fn from_reg_key<'a>(name: impl Into<Ident<Cow<'a, str>>>) -> Option<Self> {
        EntityKind::from_ident(name)
    }

    fn to_reg_key(self) -> Ident<&'static str> {
        self.ident()
    }
}
