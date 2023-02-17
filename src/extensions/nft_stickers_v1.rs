#[derive(Debug, Serialize, Deserialize)]
pub struct NFTStickerMessagePart {
    nft_address: String,
    emoji: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NFTStickerReactionPart {
    /// the event id.
    react_to: String,
    nft_address: String,
    emoji: String,
}