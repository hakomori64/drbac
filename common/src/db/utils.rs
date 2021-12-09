use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use load_dotenv::load_dotenv;
use std::env;
use anyhow::{Result, anyhow};

pub fn establish_connection() -> Result<SqliteConnection> {
    load_dotenv!();

    let database_url = env!("DATABASE_URL");
    SqliteConnection::establish(&database_url)
        .map_err(|_| anyhow!("DBへの接続に失敗しました"))
}

