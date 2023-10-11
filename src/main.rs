use axum::{routing::get, Router};
use mysql_async::{OptsBuilder, Pool};
use std::{error::Error, sync::OnceLock};
mod check_and_create;
mod individual_rank;
mod name_insert;
mod read;
mod score;
mod update;
use check_and_create::check_and_create;
use individual_rank::individual_rank;
use name_insert::name_insert;
use read::read;
use score::score;
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
    check_and_create().await;

    let app = Router::new()
        .route("/name_insert.php", get(name_insert).post(name_insert))
        .route("/update.php", get(update).put(update).post(update))
        .route("/read.php", get(read))
        .route("/score.php", get(score))
        .route("/individual_rank.php", get(individual_rank));
    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
