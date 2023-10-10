use axum::extract::Query;
use serde::Deserialize;

use crate::CELL;

pub async fn update(Query(query): Query<StatusParams>) {
    let mut conn = CELL
        .get()
        .unwrap()
        .get_conn()
        .await
        .unwrap_or_else(|e| panic!("コネクションが取れなかったんやが: {}", e));
}

#[derive(Debug, Deserialize)]
pub struct StatusParams {
    id: u32,
    score1: Option<u32>,
    score2: Option<u32>,
    speed: Option<u32>,
    stamina: Option<u32>,
    luck: Option<u32>,
    power: Option<u32>,
}
