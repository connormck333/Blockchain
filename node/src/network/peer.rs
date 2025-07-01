use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub struct Peer {
    pub address: String,
    pub writer: OwnedWriteHalf,
    pub reader: OwnedReadHalf
}

impl Peer {
    pub fn new(address: String, writer: OwnedWriteHalf, reader: OwnedReadHalf) -> Self {
        Self {
            address,
            writer,
            reader
        }
    }
}