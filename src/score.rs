use crate::CELL;
use axum::Json;
use mysql_async::{prelude::Queryable, Conn, Params, Row, Value};
use serde::Serialize;

pub async fn score() -> Json<ReturnParams> {
    let conn = CELL
        .get()
        .unwrap()
        .get_conn()
        .await
        .unwrap_or_else(|e| panic!("コネクションが取れなかったんやが: {}", e));
    let (score1, conn) = scores(conn, "score1").await;
    let (score2, _) = scores(conn, "score2").await;
    Json(ReturnParams { score1, score2 })
}

async fn scores(mut conn: Conn, column_name: &str) -> (Vec<Score>, Conn) {
    let result = conn
        .exec::<Row, _, _>(
            "select score1 from status order by ? desc limit 0, 5",
            Params::Positional(vec![Value::from(column_name)]),
        )
        .await
        .unwrap_or_else(|e| {
            eprintln!("エラーある: {}", e);
            Vec::new()
        });
    (
        result
            .iter()
            .map(|x| Score {
                score: x.get::<u32, _>(column_name).unwrap(),
                name: x.get::<String, _>("name").unwrap(),
            })
            .collect(),
        conn,
    )
}

#[derive(Debug, Serialize)]
pub struct ReturnParams {
    score1: Vec<Score>,
    score2: Vec<Score>,
}

#[derive(Debug, Default, Serialize)]
struct Score {
    score: u32,
    name: String,
}
