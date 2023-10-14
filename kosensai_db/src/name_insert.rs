use axum::extract::Query;
use mysql_async::{prelude::Queryable, Params, Value};
use serde::Deserialize;

use crate::CELL;

pub async fn name_insert(Query(query): Query<NameParams>) {
    let mut conn = CELL
        .get()
        .unwrap()
        .get_conn()
        .await
        .unwrap_or_else(|e| panic!("コネクションが取れなかったんやが: {}", e));
    conn.exec_drop(
        "insert into status (name) values(?)",
        Params::Positional(vec![Value::from(query.name)]),
    )
    .await
    .unwrap_or_else(|e| eprintln!("エラーある: {}", e));
}

#[derive(Debug, Deserialize)]
pub struct NameParams {
    name: String,
}
