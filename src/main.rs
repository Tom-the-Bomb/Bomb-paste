#![feature(once_cell)]

mod models;
mod helpers;
mod templates;

use axum::{
    error_handling::HandleErrorLayer,
    response::{Response, IntoResponse},
    routing::{get, post, get_service},
    http::StatusCode,
    extract::{ConnectInfo, Path},
    body::Body,
    Router,
    Json,
};
use mongodb::{
    options::ClientOptions,
    bson::doc,
    Collection,
    Client,
};
use std::{
    collections::HashMap,
    sync::{RwLock, OnceLock},
    net::SocketAddr,
    time::Duration,
};
use chrono::{DateTime, Utc};
use tower::{
    ServiceBuilder,
    buffer::BufferLayer,
    limit::RateLimitLayer
};
use tower_http::services::ServeDir;
use lazy_static::lazy_static;

static COLLECTION: OnceLock<Collection<models::PasteModel>> = OnceLock::new();

lazy_static! {
    static ref RATELIMITS: RwLock<HashMap<SocketAddr, DateTime<Utc>>> = RwLock::new(HashMap::new());
}

const MIN_PASTE_LENGTH: usize = 0;
const MAX_PASTE_LENGTH: usize = 500_000;

const MAX_GLOBAL_UPLOAD_RATE: u64 = 100;
const MAX_GLOBAL_UPLOAD_PER: u64 = 1;


async fn post_upload(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<models::UploadPayload>,
) -> Response {
    // handles the POST request to upload a paste

    match RATELIMITS.write() {
        Ok(mut ratelimits) => {
            if helpers::is_ratelimited(&mut *ratelimits, &addr) {
                return (
                    StatusCode::TOO_MANY_REQUESTS,
                    format!("Woah slow down, the limit is 1 request per {} seconds per ip", helpers::MAX_UPLOAD_PER),
                ).into_response()
            }
        }
        Err(_) => (),
    }

    if payload.content.len() > MIN_PASTE_LENGTH {

        if payload.content.len() > MAX_PASTE_LENGTH {
            (
                StatusCode::PAYLOAD_TOO_LARGE,
                format!("Paste content cannot be over {MAX_PASTE_LENGTH} characters."),
            ).into_response()
        } else {
            let id = helpers::generate_id(20);
            let collection = COLLECTION.get().unwrap();

            match collection.insert_one(
                models::PasteModel {
                    id: id.clone(),
                    content: payload.content,
                },
                None,
            ).await {
                Ok(_) => Json(models::PasteJsonResponse { id }).into_response(),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong when updating the DB.",
                ).into_response(),
            }
        }

    } else {
        (
            StatusCode::BAD_REQUEST,
            format!("Paste content must be at least {MIN_PASTE_LENGTH} characters in length."),
        ).into_response()
    }
}


async fn get_root(ConnectInfo(_): ConnectInfo<SocketAddr>) -> Response {
    // renders index.html, for GET / (root)
    let template = templates::Index {};
    helpers::render_template(template)
}


async fn get_help(ConnectInfo(_): ConnectInfo<SocketAddr>) -> Response {
    // renders help.html, for GET /help (help page)
    let template = templates::Help {
        min_content_length: MIN_PASTE_LENGTH,
        max_content_length: MAX_PASTE_LENGTH,
        max_upload_per: helpers::MAX_UPLOAD_PER,
    };
    helpers::render_template(template)
}


async fn get_paste(ConnectInfo(_): ConnectInfo<SocketAddr>, Path(params): Path<String>) -> Response {
    // tries to fetch a paste from DB and renders /:paste_id to display it
    let mut parts = params.split(".");
    let paste_id = parts.next().unwrap_or("not found");

    let collection = COLLECTION.get().unwrap();
    let paste_result = collection.find_one(
        doc! { "id": paste_id }, None
    ).await;

    match paste_result {
        Ok(paste) => {
            match paste {
                None => helpers::render_template(templates::NotFound {}),
                Some(paste) => helpers::render_template(
                    templates::Paste { paste_content: paste.content.as_str() }
                ),
            }
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong when fetching for the paste in the DB.",
        ).into_response()
    }
}


async fn init_mongo(config: &models::Config) -> mongodb::error::Result<()> {
    // connects to the mongo database
    let mongo_url = format!(
        "mongodb+srv://{}:{}@{}.efj2q.mongodb.net/?retryWrites=true&w=majority",
        config.mongo_username, config.mongo_password, config.mongo_cluster,
    );

    let client_options = ClientOptions::parse(mongo_url).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database(config.database_name.as_str());

    COLLECTION.set(
        database.collection::<models::PasteModel>(
            config.collection_name.as_str()
        )
    ).unwrap();
    Ok(())
}


async fn run(app: Router<Body>, port: u16) {
    // runs the webserver
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to await for SIGINT")
        });

    println!("[Server Initialized]");
    server.await.expect("Failed to start server");
}


#[tokio::main]
async fn main() {
    let config = helpers::get_config().unwrap();
    let app: Router<Body> = Router::new()
        .route("/", get(get_root))
        .route("/help", get(get_help))
        .route("/upload", post(post_upload)
            .layer(ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Something went wrong: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(
                    MAX_GLOBAL_UPLOAD_RATE,
                    Duration::from_secs(MAX_GLOBAL_UPLOAD_PER)
                ))
            )
        )
        .route("/:paste_id", get(get_paste))
        .fallback(get_service(ServeDir::new("./static/"))
            .handle_error(|err| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to serve files: {err}"),
                )
            })
        );

    println!("[App Initialized]");
    init_mongo(&config).await.unwrap();
    println!("[Connected to Mongo Database]");

    let port = config.port.parse::<u16>().unwrap();
    run(app, port).await;
}