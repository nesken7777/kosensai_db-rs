use axum::{extract::Query, routing::get, Router};
use mysql_async::{prelude::*, OptsBuilder, Pool, Row};
use serde::{de, Deserialize, Deserializer};
use std::{error::Error, fmt, str::FromStr, sync::OnceLock};

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

    let app = Router::new().route("/name_insert.php", get(name_insert));

    axum::Server::bind(&"0.0.0.0:7878".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn name_insert(Query(query): Query<NameParams>) {
    let mut conn = CELL.get().unwrap().get_conn().await.unwrap();
    conn.exec::<Row, _, _>(
        "UPDATE status SET name = :name where id = :id",
        params! {
            "name" => query.name.unwrap(),
            "id" => query.id.unwrap(),
        },
    ).await.unwrap();
}
async fn read() {}
async fn score() {}
async fn update() {}

#[derive(Debug, Deserialize)]
struct NameParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    id: Option<u32>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StatusParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    id: Option<u32>,
    score1: Option<u32>,
    score2: Option<u32>,
    speed: Option<u32>,
    stamina: Option<u32>,
    luck: Option<u32>,
    power: Option<u32>,
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s)
            .map_err(|x| de::Error::custom(x))
            .map(|x| Some(x)),
    }
}
