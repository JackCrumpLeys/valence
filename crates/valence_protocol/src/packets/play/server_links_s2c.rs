use crate::Packet;
use crate::packets::configuration::server_links_s2c::ServerLink;
use valence_binary::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct ServerLinksS2c<'a> {
    pub links: Vec<ServerLink<'a>>,
}
