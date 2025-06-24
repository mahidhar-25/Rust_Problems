// src/lib.rs

pub mod cli;
pub mod db;
pub mod models;
pub mod web;

#[macro_use]
extern crate diesel;

pub mod schema;

use crate::db::query_loader::QUERY_MAP;
use crate::models::Todo;
