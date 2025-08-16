mod controllers;
mod error;
mod seeders;
mod tables;

use std::sync::OnceLock;

use axum::routing::post;
use axum::{Router, routing::get};
use rusqlite::Connection;
use tokio::sync::Mutex;

use anyhow::{Context, Result};
use tracing::Level;
use tracing::info;

use crate::controllers::data_handler;
use crate::tables::initialize_tables;

static DB_WRITER: OnceLock<Mutex<Connection>> = OnceLock::new();
const DB_FILENAME: &str = "db.sqlite";

fn get_db_writer() -> &'static Mutex<Connection> {
    DB_WRITER.get_or_init(|| {
        let conn = Connection::open(DB_FILENAME).expect("Failed to open database");
        conn.execute("PRAGMA foreign_keys = ON;", [])
            .expect("Failed to enable foreign keys");
        let _ = conn.execute("PRAGMA journal_mode = WAL;", []);
        conn.into()
    })
}

#[macro_export]
macro_rules! db_writer {
    () => {
        std::ops::Deref::deref(&crate::get_db_writer().lock().await)
    };
}

pub fn get_db_reader() -> Result<Connection> {
    Connection::open_with_flags(DB_FILENAME, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
        .context("Failed to open read-only database")
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt::Subscriber::builder()
        // .with_file(true)
        // .with_line_number(true)
        .with_max_level(Level::DEBUG)
        .init();
    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(Level::INFO)
        .with_ansi(true)
        .init();

    initialize_tables(get_db_writer()).await?;

    let routes = Router::new()
        .route("/hello", get(|| async { "Hello!" }))
        .route("/data", post(data_handler));

    let addr = tokio::net::TcpListener::bind("0.0.0.0:23564")
        .await
        .context("Binding TCP listener failed")?;
    info!("Listening on {}", addr.local_addr()?);
    axum::serve(addr, routes).await?;

    Ok(())
}
