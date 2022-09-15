use std::{
    net::SocketAddr,
    collections::HashMap,
    env,
};
use dotenv::dotenv;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response, Html},
};

use crate::models::Config;
use crate::templates;
use chrono::{DateTime, Utc};

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub const MAX_UPLOAD_PER: u64 = 3;


pub fn generate_id(length: usize) -> String {
    // generates a random alphanumeric string for the paste id
    let mut rng = thread_rng();

    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect::<String>()
}

fn var(name: &'static str) -> Result<String, &'static str> {
    env::var(name)
        .map_err(|_| "ENV vars could not be loaded")
}

/// fetches the config ENV variables
pub fn get_config() -> Result<Config, &'static str> {
    dotenv().ok();

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

/// renders a template with a provided status code
pub fn render_template_with_status<T: Template>(
    template: T,
    status: Option<StatusCode>,
) -> Response {
    let statuscode = status
        .unwrap_or(StatusCode::OK);

    match template.render() {
        Ok(rendered) => (
            statuscode,
            Html(rendered),
        ).into_response(),
        Err(_) => {
            let error_template = templates::InternalError {};

            match error_template.render() {
                Ok(rendered) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(rendered),
                ),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html("<h1>Something really went wrong D:</h1>".to_string()),
                ),
            }.into_response()
        }
    }
}

/// renders a regular template with an OK status
pub fn render_template<T: Template>(template: T) -> Response {
    render_template_with_status(
        template,
        Some(StatusCode::OK),
    )
}

/// renders the not found (404) template
pub fn render_not_found() -> Response {
    render_template_with_status(
        templates::NotFound {},
        Some(StatusCode::NOT_FOUND),
    )
}

/// checks if a client is ratelimited by IP
pub fn is_ratelimited(
    mapping: &mut HashMap<SocketAddr, DateTime<Utc>>,
    ip: &SocketAddr,
) -> bool {
    let now = Utc::now();

    match mapping.get(ip) {
        Some(timestamp) => {
            let pass = (now - *timestamp).num_seconds() as u64 >= MAX_UPLOAD_PER;
            if pass {
                mapping.insert(*ip, now);
            }
            !pass
        }
        None => {
            mapping.insert(*ip, now);
            false
        }
    }
}