use mysql_async::{prelude::Queryable, Params};

use crate::CELL;

pub async fn check_and_create() {
    let check = CELL
        .get()
        .unwrap()
        .get_conn()
        .await
        .unwrap()
        .exec_drop("select 1 from status limit 1;", Params::Empty)
        .await;
    if check.is_err() {
        create_db().await;
    }
}

async fn create_db() {
    CELL.get()
        .unwrap()
        .get_conn()
        .await
        .unwrap()
        .exec_drop(
            "create table status (
    id int(10) unsigned not null auto_increment primary key,
    power int(10) unsigned not null default 0,
    speed int(10) unsigned not null default 0,
    stamina int(10) unsigned not null default 0,
    luck int(10) unsigned not null default 0,
    score1 int(10) unsigned not null default 0,
    score2 int(10) unsigned not null default 0,
    name text not null
);",
            Params::Empty,
        )
        .await
        .unwrap();
}
