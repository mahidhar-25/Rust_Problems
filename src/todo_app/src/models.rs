use crate::schema::todos;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub title: String,
    #[serde(default)] // If omitted in input, assume false
    pub completed: bool,
}
