// src/lib.rs

pub mod cli;
pub mod db;
pub mod models;
pub mod web;

#[macro_use]
extern crate diesel;

pub mod schema;

use crate::db::get_query;
use crate::models::Todo;
use diesel::sql_query;

pub fn get_all_todos(conn: &mut SqliteConnection) -> Vec<Todo> {
    let sql = get_query("get_all_todos");
    sql_query(sql)
        .load::<Todo>(conn)
        .expect("Failed to load todos")
}
