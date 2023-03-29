pub struct UploadRestrictionV1 {
    max_size: u64,
    max_count_per_day: u64,
    max_size_per_day: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUploadRestrictionRequestV1 {
    jwt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUploadRestrictionResponseV1 {
    #[serde(flatten)]
    restriction: UploadRestrictionV1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadRequestV1 {
    // user jwt
    jwt: String,
    // the file name
    file_name: String,
    // the file size
    file_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponseV1 {
    upload_to_url: String,
    upload_token: String,
}
