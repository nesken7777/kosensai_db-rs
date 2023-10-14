use axum::{extract::Query, Json};
use mysql_async::{prelude::Queryable, Params, Row, Value};
use serde::{Deserialize, Serialize};

use crate::CELL;

pub async fn read(Query(query): Query<ReadParams>) -> Json<ReturnParams> {
    let mut conn = CELL
        .get()
        .unwrap()
        .get_conn()
        .await
        .unwrap_or_else(|e| panic!("コネクションが取れなかったんやが: {}", e));
    let mut result = conn
        .exec::<Row, _, _>(
            "select * from status where id = ?",
            Params::Positional(vec![Value::from(query.id)]),
        )
        .await
        .unwrap_or_else(|e| {
            eprintln!("エラーある: {}", e);
            Vec::new()
        });
    let result = result.remove(0);
    Json(ReturnParams {
        id: result.get("id").unwrap(),
        score1: result.get("score1").unwrap(),
        score2: result.get("score2").unwrap(),
        speed: result.get("speed").unwrap(),
        stamina: result.get("stamina").unwrap(),
        luck: result.get("luck").unwrap(),
        power: result.get("power").unwrap(),
        name: result.get("name").unwrap(),
    })
}

#[derive(Debug, Deserialize)]
pub struct ReadParams {
    id: u32,
}

#[derive(Debug, Serialize)]
pub struct ReturnParams {
    id: u32,
    score1: u32,
    score2: u32,
    speed: u32,
    stamina: u32,
    luck: u32,
    power: u32,
    name: String,
}
