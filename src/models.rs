use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub mongo_username: String,
    pub mongo_password: String,
    pub mongo_cluster: String,
    pub database_name: String,
    pub collection_name: String,
    pub port: String,
}


#[derive(Debug, Clone, Deserialize)]
pub struct UploadPayload {
    pub content: String,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PasteModel {
    pub id: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PasteJsonResponse {
    pub id: String,
}


#[derive(Debug, Clone, Deserialize)]
pub struct GetRawQuery {
    pub raw: Option<bool>,
}