mod config;
mod errors;
mod api;
mod dao;
mod variables;
mod logger;

use std::fs::File;
use std::io::Write;

use clap::Parser;
use config::Config;

use crate::errors::CustomError;
use axum::{extract::Extension, response::Json, routing::get, Router};
use std::net::SocketAddr;
use std::process::exit;
use axum::extract::Path;
use db::User;

use tracing::{error, info, warn};

pub use variables::*;

#[derive(Debug, Parser)]
#[clap(
name = env!("CARGO_PKG_NAME"),
about = env!("CARGO_PKG_DESCRIPTION"),
version = env!("CARGO_PKG_VERSION"),
)]
struct Cli {
    #[clap(short = 'c', long = "config", help = "Configuration file path", default_value = DEFAULT_CONFIG_FILE, display_order = 1)]
    config: String,
    #[clap(short = 'd', long = "database", help = "Postgres database path", default_value = DEFAULT_DATABASE_FILE, display_order = 2)]
    database: String,
}

#[tokio::main]
async fn main() {
    logger::init();

    let args = Cli::parse();

    let config_path = args.config;
    let database_url = args.database;

    // if configuration file doesn't exist, create a default one
    if !Path::new(&config_path).exists() {
        warn!("Configuration file doesn't exists.");

        let mut file = File::create(config_path)
            .expect("notrace - Failed to create configuration file");

        file.write_all(DEFAULT_CONFIG_CONTENT)
            .expect("notrace - Failed to write default configuration to config file");

        info!("Created configuration file. Exiting...");

        std::process::exit(0);
    }

    let config = Config::parse(&config_path).expect("notrace - Failed to parse configuration file");

    // if storage directory doesn't exist, create it
    if !Path::new(&config.storage.database_url).exists() {}

    let pool = db::create_pool(&config.storage.database_url);

    let host = if config.http.enable_https {
        format!("{}:{}", config.http.host, config.http.https_port)
    } else {
        format!("{}:{}", config.http.host, config.http.http_port)
    };

    if config.http.enable_https {
        if !Path::new(&config.http.tls_cert).exists() || !Path::new(&config.http.tls_key).exists() {
            error!("TLS cert or/and key file not found!");
            exit(1);
        }
    }

    info!("🚀 Server has launched on http://{host}");

    // change the type from Vec<String> to Vec<HeaderValue> so that the http server can correctly detect CORS hosts

    let app = Router::new()
        .route("/", get(users))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service())
        .await
        .unwrap();
}

async fn users(Extension(pool): Extension<db::Pool>) -> Result<Json<Vec<User>>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users()
        .bind(&client)
        .all()
        .await?;

    Ok(Json(users))
}


