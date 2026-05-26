use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Announce {
        node_id: [u8; 32],
        listen_addr: String,
    },
    Text {
        from: [u8; 32],
        content: String,
        signature: Vec<u8>,
    },
    Ping { nonce: u64 },
    Pong { nonce: u64 },
}

impl Message {
    pub fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        Ok(serde_json::from_slice(bytes)?)
    }
}
