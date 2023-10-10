use std::collections::HashMap;

use axum::extract::Query;
use mysql_async::{params, prelude::Queryable, Params, Row, Value};
use serde::Deserialize;

use crate::CELL;

pub async fn name_insert(Query(query): Query<NameParams>) {
    let mut conn = CELL
        .get()
        .unwrap()
        .get_conn()
        .await
        .unwrap_or_else(|e| panic!("コネクションが取れなかったんやが: {}", e));
    conn.exec::<Row, _, _>(
        "insert into status (power,score1,score2,speed,stamina,luck,name) VALUES(:power,:score1,:score2,:speed,:stamina,:luck,:name)",
        Params::Named(HashMap::from([
                (Vec::from("power"), Value::from(0)),
                (Vec::from("score1"), Value::from(0)),
                (Vec::from("score2"), Value::from(0)),
                (Vec::from("speed"), Value::from(0)),
                (Vec::from("stamina"), Value::from(0)),
                (Vec::from("luck"), Value::from(0)),
                (Vec::from("name"), Value::from(query.name))
                ]))
    )
    .await
    .unwrap();
}

#[derive(Debug, Deserialize)]
pub struct NameParams {
    name: String,
}
