use std::{env, fs};
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
    let cwd = get_cwd();
    println!("{}", cwd);
    let config_fp = fs::read_to_string(format!("{cwd}/config.toml"))
        .expect("Config could not be loaded");

    from_str(config_fp.as_str()).unwrap()
}

pub fn get_cwd() -> String {
    let path = env::current_dir();

    match path {
        Ok(res) => res.to_str().unwrap_or("").to_string(),
        Err(_) => "".to_string(),
    }
}