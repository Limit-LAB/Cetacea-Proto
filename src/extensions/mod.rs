use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Extensions {
    // user methods
    #[doc = include_str!("../../docs/extensions/user_v1.md")]
    UserV1,

    // file methods
    #[doc = include_str!("../../docs/extensions/upload_v1.md")]
    UploadV1,

    // room methods
    #[doc = include_str!("../../docs/extensions/room_v1.md")]
    RoomV1,
    #[doc = include_str!("../../docs/extensions/dao_v1.md")]
    DAOV1,

    // message types
    #[doc = include_str!("../../docs/extensions/mmm_v1.md")]
    MultiMediaMessageV1,
    #[doc = include_str!("../../docs/extensions/NFT_stickers_v1.md")]
    NFTStickersV1,

    // message methods
    #[doc = include_str!("../../docs/extensions/RTC_v1.md")]
    RTCV1,

    // login methods
    #[doc = include_str!("../../docs/extensions/jwt_login.md")]
    JWTLogin,
    #[doc = include_str!("../../docs/extensions/wallet_login.md")]
    WalletLogin,

    // backup methods
    #[doc = include_str!("../../docs/extensions/backup_s3.md")]
    BackupS3,
    #[doc = include_str!("../../docs/extensions/backup_arweave.md")]
    BackupArweave,
}

pub mod mmm_v1;
pub mod room_v1;
pub mod user_v1;
pub mod upload_v1;

#[derive(Debug, Serialize, Deserialize)]

pub struct CheckSupportedExtensionsRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckSupportedExtensionsResponse {
    pub extensions: Vec<Extensions>,
}
