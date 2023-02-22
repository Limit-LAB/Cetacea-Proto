use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTStickerMessagePart {
    pub nft_address: String,
    pub emoji: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTStickerReactionPart {
    /// the event id.
    pub react_to: String,
    pub nft_address: String,
    pub emoji: String,
}
