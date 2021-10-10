use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use anyhow::{Result, anyhow};

pub fn establish_connection() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    SqliteConnection::establish(&database_url)
        .map_err(|_| anyhow!("DBへの接続に失敗しました"))
}

