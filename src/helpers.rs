use std::env;
use dotenv::dotenv;
use crate::models::Config;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn generate_id(length: usize) -> String {
    let mut rng = thread_rng();

    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect::<String>()
}

pub fn get_config() -> Result<Config, &'static str> {
    drop(dotenv());

    const ERR_MSG: &str = "ENV vars could not be loaded";
    let config = Config {
        mongo_username: env::var("MONGO_USERNAME")
            .map_err(|_| ERR_MSG)?,
        mongo_password: env::var("MONGO_PASSWORD")
            .map_err(|_| ERR_MSG)?,
        mongo_cluster: env::var("MONGO_CLUSTER")
            .map_err(|_| ERR_MSG)?,
        database_name: env::var("DATABASE_NAME")
            .map_err(|_| ERR_MSG)?,
        collection_name: env::var("COLLECTION_NAME")
            .map_err(|_| ERR_MSG)?,
    };

    Ok(config)
}