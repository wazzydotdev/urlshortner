use rand::{Rng, RngExt, distr::Alphanumeric};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;

async fn get_url(pool: &SqlitePool, id: &str) -> Option<String> {
    sqlx::query_scalar("SELECT url FROM urls WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .unwrap()
}
async fn add_url(pool: &SqlitePool, url: &str) -> String {
    // rng cant cross an await so it stays in this block and gets dropped early
    let id: String = {
        let mut rng = rand::rng();
        (0..4).map(|_| rng.sample(Alphanumeric) as char).collect()
    };

    sqlx::query("INSERT INTO urls (id, url) VALUES (?, ?)")
        .bind(&id)
        .bind(url)
        .execute(pool)
        .await
        .unwrap();

    // give the id back so the caller can show the short link
    id
}

pub async fn handler(id: &str, url: &str, operation: &str) -> Option<String> {
    let opts = SqliteConnectOptions::from_str("sqlite://url.db")
        .unwrap()
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(opts).await.unwrap();

    // make the table on first run so inserts dont crash
    sqlx::query("CREATE TABLE IF NOT EXISTS urls (id TEXT PRIMARY KEY, url TEXT NOT NULL)")
        .execute(&pool)
        .await
        .unwrap();

    if operation == "pullinfo" {
        get_url(&pool, id).await // return the actual url or none if its not there
    } else if operation == "addurl" {
        let id = add_url(&pool, url).await;
        Some(id)
    } else {
        None
    }
}
