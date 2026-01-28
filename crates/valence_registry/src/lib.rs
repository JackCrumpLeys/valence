#![doc = include_str!("../README.md")]

pub mod codec;
pub mod impls;
pub mod tags;

use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use indexmap::map::Entry;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use tracing::{error, warn};
use valence_binary::registry_id::RegistryId;
use valence_ident::Ident;
use valence_nbt::serde::ser::CompoundSerializer;

use crate::codec::{RegistryCodec, RegistryValue};

pub use impls::*;

/// A plugin that initializes and manages all Minecraft server registries.
///
/// This plugin adds the following sub-plugins:
/// - [`codec::RegistryCodecPlugin`]
/// - [`tags::TagsRegistryPlugin`]
/// - All individual registry plugins defined in [`impls`].
pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) {
        // The set for updating registry caches (packets).
        app.configure_sets(PostUpdate, RegistrySet);

        // Core registry infrastructure
        app.add_plugins((codec::RegistryCodecPlugin, tags::TagsRegistryPlugin));

        // Register all data-driven registries
        impls::add_registry_plugins(app);
    }
}

/// The [`SystemSet`] where the registry caches (e.g. `RegistryCodec`, `TagsRegistry`)
/// are rebuilt if the underlying registries have changed.
///
/// Systems that modify registries should run *before* this set.
/// This set lives in [`PostUpdate`].
#[derive(SystemSet, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RegistrySet;

/// A generic plugin that manages the lifecycle of a specific [`Registry<T>`].
///
/// This plugin:
/// 1. Initializes the [`Registry<T>`] resource.
/// 2. Loads default values from the [`RegistryCodec`] during [`PreStartup`].
/// 3. Syncs changes from the [`Registry<T>`] back to the [`RegistryCodec`] during [`PostUpdate`].
pub struct RegistryManagerPlugin<T>(PhantomData<T>);

impl<T> Default for RegistryManagerPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: RegistryItem + Debug> Plugin for RegistryManagerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Registry<T>>()
            .add_systems(PreStartup, load_defaults::<T>)
            .add_systems(PostUpdate, sync_registry_to_codec::<T>.in_set(RegistrySet));
    }
}

/// System to load default registry values from the vanilla codec.
fn load_defaults<T: RegistryItem + Debug>(mut reg: ResMut<Registry<T>>, codec: Res<RegistryCodec>) {
    let key = T::KEY;

    if let Some(values) = codec.registry(key) {
        for value in values {
            match T::deserialize(value.element.clone()) {
                Ok(item) => {
                    // We insert directly to preserve the vanilla ID order if possible
                    reg.insert(value.name.clone(), item);
                }
                Err(e) => {
                    error!(
                        "Failed to deserialize registry item '{}' in registry '{}': {:#}",
                        value.name, key, e
                    );
                }
            }
        }
    } else {
        warn!(
            "Registry '{}' not found in default RegistryCodec. This registry will start empty.",
            key
        );
    }
}

/// System to sync registry changes back to the RegistryCodec for new client connections.
fn sync_registry_to_codec<T: RegistryItem + Debug>(
    reg: Res<Registry<T>>,
    mut codec: ResMut<RegistryCodec>,
) {
    if reg.is_changed() {
        let values = codec.registry_mut(T::KEY);
        values.clear();

        for (name, item) in &reg.items {
            match item.serialize(CompoundSerializer) {
                Ok(compound) => {
                    values.push(RegistryValue {
                        name: name.clone(),
                        element: compound,
                    });
                }
                Err(e) => {
                    error!(
                        "Failed to serialize registry item '{}' in registry '{}': {:#}",
                        name,
                        T::KEY,
                        e
                    );
                }
            }
        }
    }
}

/// A generic container for registry items.
///
/// This resource maintains an ordered mapping between [`Ident`]s (names) and values `T`.
/// It supports lookup by name or by numerical index (via [`RegistryId`]).
///
/// You shouldnt mutate this registry while clients are connected, as removing or
///
/// # Type Parameters
///
/// * `T`: The type of value stored in the registry.
#[derive(Debug, Resource, Clone)]
pub struct Registry<T> {
    /// The underlying storage. `IndexMap` is used to preserve insertion order,
    /// which maps directly to the integer ID of the entry.
    items: IndexMap<Ident<String>, T>,
}

impl<T> Default for Registry<T> {
    fn default() -> Self {
        Self {
            items: IndexMap::new(),
        }
    }
}

impl<T> Registry<T> {
    /// Creates a new, empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a new value into the registry with the given name.
    ///
    /// If an item with the same name already exists, it is **not** replaced,
    /// and the existing ID is returned (to prevent accidental ID shifting).
    ///
    /// Returns the [`RegistryId`] of the inserted (or existing) item.
    pub fn insert(&mut self, name: impl Into<Ident<String>>, item: T) -> RegistryId<T> {
        let name = name.into();
        let len = self.items.len();

        match self.items.entry(name) {
            Entry::Occupied(entry) => RegistryId::new(entry.index() as i32),
            Entry::Vacant(entry) => {
                entry.insert(item);
                RegistryId::new(len as i32)
            }
        }
    }

    /// Overwrites an item in the registry, or inserts it if it doesn't exist.
    ///
    /// Returns the [`RegistryId`] of the item.
    pub fn set(&mut self, name: impl Into<Ident<String>>, item: T) -> RegistryId<T> {
        let name = name.into();
        let len = self.items.len();

        match self.items.entry(name) {
            Entry::Occupied(mut entry) => {
                entry.insert(item);
                RegistryId::new(entry.index() as i32)
            }
            Entry::Vacant(entry) => {
                entry.insert(item);
                RegistryId::new(len as i32)
            }
        }
    }

    /// Removes an item from the registry by name.
    ///
    /// **Warning:** This shifts the IDs of all subsequent items. Dont use if
    /// clients are connected
    pub fn remove(&mut self, name: Ident<&str>) -> Option<T> {
        self.items.shift_remove(name.as_str())
    }

    /// Clears the registry.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns a reference to the item with the given name.
    pub fn get(&self, name: Ident<&str>) -> Option<&T> {
        self.items.get(name.as_str())
    }

    /// Returns a mutable reference to the item with the given name.
    pub fn get_mut(&mut self, name: Ident<&str>) -> Option<&mut T> {
        self.items.get_mut(name.as_str())
    }

    /// Returns a reference to the item with the given [`RegistryId`].
    pub fn get_by_id(&self, id: RegistryId<T>) -> Option<&T> {
        self.items.get_index(id.get() as usize).map(|(_, v)| v)
    }

    /// Returns a mutable reference to the item with the given [`RegistryId`].
    ///
    /// **Warning:**  Dont use if clients are connected
    pub fn get_mut_by_id(&mut self, id: RegistryId<T>) -> Option<&mut T> {
        self.items.get_index_mut(id.get() as usize).map(|(_, v)| v)
    }

    /// Looks up the [`RegistryId`] for a given name.
    pub fn index_of(&self, name: Ident<&str>) -> Option<RegistryId<T>> {
        self.items
            .get_index_of(name.as_str())
            .map(|i| RegistryId::new(i as i32))
    }

    /// Iterates over all items in the registry.
    ///
    /// Yields `(RegistryId<T>, Ident<&str>, &T)`.
    pub fn iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = (RegistryId<T>, Ident<&str>, &T)> + ExactSizeIterator + '_
    {
        self.items
            .iter()
            .enumerate()
            .map(|(i, (k, v))| (RegistryId::new(i as i32), k.as_str_ident(), v))
    }

    /// Iterates over all items in the registry mutably.
    ///
    /// Yields `(RegistryId<T>, Ident<&str>, &mut T)`.
    pub fn iter_mut(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = (RegistryId<T>, Ident<&str>, &mut T)> + ExactSizeIterator + '_
    {
        self.items
            .iter_mut()
            .enumerate()
            .map(|(i, (k, v))| (RegistryId::new(i as i32), k.as_str_ident(), v))
    }

    /// Returns the number of items in the registry.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T> Index<RegistryId<T>> for Registry<T> {
    type Output = T;

    fn index(&self, index: RegistryId<T>) -> &Self::Output {
        self.get_by_id(index)
            .unwrap_or_else(|| panic!("invalid registry id: {}", index.get()))
    }
}

impl<T> IndexMut<RegistryId<T>> for Registry<T> {
    fn index_mut(&mut self, index: RegistryId<T>) -> &mut Self::Output {
        let idx_val = index.get();
        self.get_mut_by_id(index)
            .unwrap_or_else(|| panic!("invalid registry id: {}", idx_val))
    }
}

impl<'a, T> Index<Ident<&'a str>> for Registry<T> {
    type Output = T;

    fn index(&self, index: Ident<&'a str>) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("missing registry item: {}", index))
    }
}

impl<'a, T> IndexMut<Ident<&'a str>> for Registry<T> {
    fn index_mut(&mut self, index: Ident<&'a str>) -> &mut Self::Output {
        let name_str = index.as_str();
        self.get_mut(index)
            .unwrap_or_else(|| panic!("missing registry item: {}", name_str))
    }
}

impl<'a, T> IntoIterator for &'a Registry<T> {
    type Item = (RegistryId<T>, Ident<&'a str>, &'a T);
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Registry<T> {
    type Item = (RegistryId<T>, Ident<&'a str>, &'a mut T);
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
