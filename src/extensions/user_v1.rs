use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum VisibilityV1 {
    Public,
    /// require to login to see the user.
    OnlyToFriends,
    NotAllow,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoFieldV1<T> {
    visibility: VisibilityV1,
    field: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserHeaderV1 {
    /// the user id.
    pub user_id: String,
    /// the user server.
    pub user_server: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoV1 {
    /// the user header
    #[serde(flatten)]
    pub header: UserHeaderV1,
    /// the user name.
    pub user_name: UserInfoFieldV1<String>,
    /// the user avatar.
    pub avatar: UserInfoFieldV1<String>,
    /// the user bio.
    pub bio: UserInfoFieldV1<String>,
    /// the user visibility.
    pub visibility: VisibilityV1,
    /// add to group
    pub add_to_group: VisibilityV1,
    /// externsion
    #[serde(flatten)]
    pub extension: BTreeMap<String, UserInfoFieldV1<serde_json::Value>>,
}

#[test]
fn test_userv1_extention() {
    let user: serde_json::Value = serde_json::from_str(
        r#"
    {
        "user_id": "test",
        "user_server": "test",
        "user_name": {
            "visibility": "Public",
            "field": "test"
        },
        "avatar": {
            "visibility": "Public",
            "field": "test"
        },
        "bio": {
            "visibility": "Public",
            "field": "test"
        },
        "visibility": "Public",
        "add_to_group": "Public",
        "test": {
            "visibility": "Public",
            "field": "test"
        }
    }
    "#,
    )
    .unwrap();
    let user: UserInfoV1 = serde_json::from_value(user).unwrap();
    println!("{:#?}", user)
}

#[derive(Debug, Serialize, Deserialize)]
struct LastReadV1 {
    room_id: String,
    event_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Friend {
    alias: String,
    user: UserInfoV1,
    last_login_ts: u64,
    friend_since: u64,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserRecordV1 {
    last_login_ts: u64,
    pinged_rooms: Vec<String>,
    last_reads: Vec<LastReadV1>,
    user_info: UserInfoV1,
    blocked_users: Vec<UserHeaderV1>,
    friends: Vec<Friend>,
}
