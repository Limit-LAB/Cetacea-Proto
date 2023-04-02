use serde::{Deserialize, Serialize};

use crate::{
    extensions::user_v1::{UserHeaderV1, UserInfoV1, UserRecordV1, UserStatusNotifyEventV1},
    RequestMessage, Response,
};

// user info

/// | limit | yes/no |
/// | --- | --- |
/// | rate limit   | no |
/// | require auth | optional |
///
/// POST `/user/user_info_v1/get`
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserInfoRequestV1 {
    #[serde(flatten)]
    user_header: UserHeaderV1,
}
impl RequestMessage for GetUserInfoRequestV1 {}

/// | limit | yes/no |
/// | --- | --- |
/// | rate limit   | no |
/// | require auth | yes |
///
/// POST `/user/user_info_v1/set`
#[derive(Debug, Serialize, Deserialize)]
pub struct SetUserInfoRequestV1 {
    #[serde(flatten)]
    user_info: UserInfoV1,
}
impl RequestMessage for SetUserInfoRequestV1 {}

/// mainly used for streaming client
/// POST `/user/status_notify_v1/notify`
#[derive(Debug, Serialize, Deserialize)]
pub struct UserStatusNotifyEventRequestV1 {
    event: UserStatusNotifyEventV1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserInfoResponseV1 {
    #[serde(flatten)]
    user_info: UserInfoV1,
}
impl Response for GetUserInfoResponseV1 {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetUserInfoResponseV1 {}
impl Response for SetUserInfoResponseV1 {}

// user record

/// | limit | yes/no |
/// | --- | --- |
/// | rate limit   | no |
/// | require auth | yes |
///
/// POST `/user/user_record_v1/get`
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSelfUserRecordRequestV1 {}
impl RequestMessage for GetSelfUserRecordRequestV1 {}

/// | limit | yes/no |
/// | --- | --- |
/// | rate limit   | no |
/// | require auth | yes |
///
/// POST `/user/user_record_v1/set`
#[derive(Debug, Serialize, Deserialize)]
pub struct SetSelfUserRecordRequestV1 {
    #[serde(flatten)]
    user_record: UserRecordV1,
}
impl RequestMessage for SetSelfUserRecordRequestV1 {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSelfUserRecordResponseV1 {
    #[serde(flatten)]
    user_record: UserRecordV1,
}
impl Response for GetSelfUserRecordResponseV1 {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetSelfUserRecordResponseV1 {}
impl Response for SetSelfUserRecordResponseV1 {}
