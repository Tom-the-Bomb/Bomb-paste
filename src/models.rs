use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct Config {
    pub mongo_username: String,
    pub mongo_password: String,
    pub mongo_cluster: String,
    pub database_name: String,
    pub collection_name: String,
    pub port: String,
}


#[derive(Deserialize)]
pub struct UploadPayload {
    pub content: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PasteModel {
    pub id: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct PasteJsonResponse {
    pub id: String,
}