#![feature(once_cell)]

mod models;
mod helpers;
mod templates;

use axum::{
    error_handling::HandleErrorLayer,
    response::{Html, IntoResponse},
    routing::{get, post, get_service},
    http::StatusCode,
    extract::Path,
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
use askama::Template;
use std::sync::OnceLock;
use std::net::SocketAddr;
use std::time::Duration;
use tower::{
    ServiceBuilder,
    buffer::BufferLayer,
    limit::RateLimitLayer
};
use tower_http::services::ServeDir;

static COLLECTION: OnceLock<Collection<models::PasteModel>> = OnceLock::new();


async fn post_upload(Json(payload): Json<models::UploadPayload>) -> impl IntoResponse {
    if payload.content.len() > 0 {
        let id = helpers::generate_id(20);
        let collection = COLLECTION.get().unwrap();

        collection.insert_one(
            models::PasteModel {
                id: id.clone(),
                content: payload.content,
            },
            None,
        ).await.unwrap();

        Json(models::PasteJsonResponse { id }).into_response()
    } else {
        StatusCode::BAD_REQUEST.into_response()
    }
}


async fn get_root() -> Html<String> {
    let template = templates::Index {};
    Html(template.render().unwrap_or_else(|_| "Woops something went wrong".to_string()))
}

async fn get_help() -> Html<String> {
    let template = templates::Help {};
    Html(template.render().unwrap_or_else(|_| "Woops something went wrong".to_string()))
}

async fn get_paste(Path(params): Path<String>) -> Html<String> {

    let mut parts = params.split(".");
    let paste_id = parts.next().unwrap_or("not found");

    let collection = COLLECTION.get().unwrap();
    let paste = collection.find_one(
        doc! { "id": paste_id }, None
    ).await.unwrap();

    match paste {
        None => Html(
            templates::NotFound {}
            .render()
            .unwrap_or_else(|_| "Woops something went wrong".to_string())
        ),
        Some(paste) => Html(
            templates::Paste { paste_content: paste.content.as_str() }
            .render()
            .unwrap_or_else(|_| "Woops something went wrong".to_string())
        ),
    }
}


async fn init_mongo(config: &models::Config) -> mongodb::error::Result<()> {
    let mongo_url = format!(
        "mongodb+srv://{}:{}@{}.efj2q.mongodb.net/?retryWrites=true&w=majority",
        config.mongo_username, config.mongo_password, config.mongo_cluster,
    );

    let client_options = ClientOptions::parse(mongo_url).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database(config.database_name.as_str());

    COLLECTION.set(database.collection::<models::PasteModel>(config.collection_name.as_str())).unwrap();
    Ok(())
}


async fn run(app: Router<Body>, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
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
                .layer(RateLimitLayer::new(1, Duration::from_secs(3)))
            )
        )
        .route("/:paste_id", get(get_paste))
        .fallback(get_service(ServeDir::new("./static/"))
        .handle_error(|err| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to serve files: {err}"),
            )
        }));

    println!("[App Initialized]");
    init_mongo(&config).await.unwrap();
    println!("[Connected to Mongo Database]");

    let port = config.port.parse::<u16>().unwrap();
    run(app, port).await;
}
