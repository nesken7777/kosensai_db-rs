async fn score() {
    let mut conn = CELL
        .get()
        .unwrap()
        .get_conn()
        .await
        .unwrap_or_else(|e| panic!("コネクションが取れなかったんやが: {}", e));
    conn.exec("")
}
