use axum::{extract::Query, Json};
use mysql_async::{prelude::Queryable, Params, Row, Value};
use serde::{Deserialize, Serialize};

use crate::CELL;

pub async fn individual_rank(Query(query): Query<ReadParams>) -> Json<ReturnParams> {
    let mut conn = CELL
        .get()
        .unwrap()
        .get_conn()
        .await
        .unwrap_or_else(|e| panic!("コネクションが取れなかったんやが: {}", e));
    let mut result = conn.exec::<Row,_,_>("SELECT RANKED.SCORE_RANK FROM (SELECT RANK() OVER(ORDER BY score1 DESC) AS SCORE_RANK, id FROM STATUS) AS RANKED WHERE id = ?", Params::Positional(vec![Value::from( query.id)])).await.unwrap_or_else(|e| {eprintln!("エラーある: {}", e);Vec::new()});
    let rank = result.remove(0).get::<u32, _>("SCORE_RANK").unwrap();
    Json(ReturnParams { rank })
}

#[derive(Debug, Deserialize)]
pub struct ReadParams {
    id: u32,
}

#[derive(Debug, Serialize)]
pub struct ReturnParams {
    rank: u32,
}
