use axum::{routing::get, Router};
use mysql_async::{OptsBuilder, Pool};
use std::{
    env::args,
    error::Error,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::OnceLock,
};
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
    let port_num = args()
        .nth(1)
        .map_or(80, |num_str| num_str.parse::<u16>().unwrap_or(80));
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
    axum::Server::bind(&SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(0, 0, 0, 0),
        port_num,
    )))
    .serve(app.into_make_service())
    .await
    .unwrap();
    Ok(())
}
