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

fn var(name: &'static str) -> Result<String, &'static str> {
    env::var(name)
        .map_err(|_| "ENV vars could not be loaded")
}

pub fn get_config() -> Result<Config, &'static str> {
    drop(dotenv());

    let config = Config {
        mongo_username: var("MONGO_USERNAME")?,
        mongo_password: var("MONGO_PASSWORD")?,
        mongo_cluster: var("MONGO_CLUSTER")?,
        database_name: var("DATABASE_NAME")?,
        collection_name: var("COLLECTION_NAME")?,
        port: var("PORT")?,
    };

    Ok(config)
}