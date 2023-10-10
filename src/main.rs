use axum::{extract::Query, routing::get, Router};
use mysql_async::{OptsBuilder, Pool};
use serde::Deserialize;
use std::{error::Error, sync::OnceLock};
mod name_insert;
mod update;
use name_insert::name_insert;
use update::update;
static CELL: OnceLock<Pool> = OnceLock::new();

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    let opts = OptsBuilder::default()
        .user(Some("uk"))
        .pass(Some("1234"))
        .ip_or_hostname("localhost")
        .tcp_port(3306)
        .db_name(Some("game"));

    CELL.get_or_init(|| Pool::new(opts));

    let app = Router::new()
        .route("/name_insert.php", get(name_insert))
        .route("/update.php", get(update).put(update).post(update));
    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn read() {}
async fn score() {}
