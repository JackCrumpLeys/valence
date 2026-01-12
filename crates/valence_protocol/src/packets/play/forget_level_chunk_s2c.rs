use crate::ChunkPos;
use crate::Packet;
use valence_binary::{Decode, Encode};

#[derive(Copy, Clone, Debug, Decode, Packet)]
pub struct ForgetLevelChunkS2c {
    pub pos: ChunkPos,
}

// Note: The order is inverted, because the client reads this packet as
// one big-endian Long, with Z being the upper 32 bits.
// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Unload_Chunk
impl Encode for ForgetLevelChunkS2c {
    fn encode(&self, mut w: impl std::io::Write) -> anyhow::Result<()> {
        self.pos.z.encode(&mut w)?;
        self.pos.x.encode(&mut w)
    }
}
