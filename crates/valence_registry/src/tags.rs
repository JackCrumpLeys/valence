use std::borrow::Cow;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::RegistrySet;

#[derive(Debug, Resource, Default)]
pub struct TagsRegistry {
    pub registries: RegistryMap,
    cached_packet: Vec<u8>,
}

// pub(super) fn build(app: &mut App) {
//     app.init_resource::<TagsRegistry>()
//         .add_systems(PreStartup, init_tags_registry)
//         .add_systems(PostUpdate, cache_tags_packet.in_set(RegistrySet));
// }

// impl TagsRegistry {
//     fn build_synchronize_tags(&self) -> UpdateTagsS2c<'_> {
//         UpdateTagsS2c {
//             groups: Cow::Borrowed(&self.registries),
//         }
//     }
//     /// Returns bytes of the cached [`UpdateTagsS2c`] packet.
//     pub fn sync_tags_packet(&self) -> &[u8] {
//         &self.cached_packet
//     }
// }

impl TagsRegistry {
    pub fn default_tags() -> Self {
        serde_json::from_str::<RegistryMap>(include_str!("../extracted/tags.json"))
            .expect("tags.json must have expected structure")
    }
}

fn init_tags_registry(mut tags: ResMut<TagsRegistry>) {
    let registries = serde_json::from_str::<RegistryMap>(include_str!("../extracted/tags.json"))
        .expect("tags.json must have expected structure");

    tags.registries = registries;
}

// pub(crate) fn cache_tags_packet(server: Res<Server>, tags: ResMut<TagsRegistry>) {
//     if tags.is_changed() {
//         let tags = tags.into_inner();
//         let packet = tags.build_synchronize_tags();
//         let mut bytes = vec![];
//         let mut writer = PacketWriter::new(&mut bytes, server.compression_threshold());
//
//         writer.write_packet(&packet);
//         tags.cached_packet = bytes;
//     }
// }
