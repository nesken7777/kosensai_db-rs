use axum::extract::Query;
use mysql_async::{prelude::Queryable, Params, Value};
use serde::Deserialize;

use crate::CELL;

pub async fn update(Query(query): Query<StatusParams>) {
    if let Some(score1) = query.score1 {
        update_status(query.id, "score1", score1).await;
    }
    if let Some(score2) = query.score2 {
        update_status(query.id, "score2", score2).await;
    }
    if let Some(speed) = query.speed {
        update_status(query.id, "speed", speed).await;
    }
    if let Some(stamina) = query.stamina {
        update_status(query.id, "stamina", stamina).await;
    }
    if let Some(luck) = query.luck {
        update_status(query.id, "luck", luck).await;
    }
    if let Some(power) = query.power {
        update_status(query.id, "power", power).await;
    }
}

// &'staticによって定数であることを保証している！SQLインジェクションができないため安全！
async fn update_status(id: u32, status: &'static str, value: u32) {
    let mut conn = CELL
        .get()
        .unwrap()
        .get_conn()
        .await
        .unwrap_or_else(|e| panic!("コネクションが取れなかったんやが: {}", e));
    let query = {
        let mut query = String::with_capacity(50);
        query.push_str("select ");
        query.push_str(status);
        query.push_str(" from status where id = ?");
        query
    };
    let result = match status {
        "score1" | "score2" => 0,
        _ => conn
            .exec::<u32, _, _>(query, Params::Positional(vec![Value::from(id)]))
            .await
            .unwrap_or_else(|e| {
                eprintln!("エラーある: {}", e);
                Vec::new()
            })
            .remove(0),
    };
    let query = {
        let mut query = String::with_capacity(50);
        query.push_str("update status set ");
        query.push_str(status);
        query.push_str("=? where id = ?");
        query
    };
    conn.exec_drop(
        query,
        Params::Positional(vec![Value::from(result + value), Value::from(id)]),
    )
    .await
    .unwrap_or_else(|e| eprintln!("エラーある: {}", e));
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
