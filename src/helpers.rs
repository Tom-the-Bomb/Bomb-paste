use std::fs;
use toml::from_str;
use crate::models::Config;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn generate_id(length: usize) -> String {
    let mut rng = thread_rng();

    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect::<String>()
}

pub fn get_config() -> Config {
    let config_fp = fs::read_to_string("app/config.toml")
        .expect("Config could not be loaded");

    from_str(config_fp.as_str()).unwrap()
}