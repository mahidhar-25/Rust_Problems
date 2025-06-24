use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub mod query_loader;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

static SQL_QUERIES: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let queries = query_loader::load_queries("src/db/queries/todos.sql");
    Mutex::new(queries)
});

pub fn get_query(name: &str) -> String {
    SQL_QUERIES
        .lock()
        .unwrap()
        .get(name)
        .expect(&format!("Query `{}` not found", name))
        .clone()
}
