use crate::models::{NewTodo, Todo};
use crate::sql::QUERY_MAP;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Bool, Text};
use diesel::sqlite::SqliteConnection;

pub fn create_todo(conn: &mut SqliteConnection, new: NewTodo) -> QueryResult<usize> {
    let query = QUERY_MAP.get("create_todo").expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(&new.title)
        .bind::<Bool, _>(new.completed)
        .execute(conn)
}

pub fn get_all_todos(conn: &mut SqliteConnection) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP.get("get_all_todos").expect("Missing query");
    sql_query(query).load::<Todo>(conn)
}
pub fn get_todo_by_id(conn: &mut SqliteConnection, id: i32) -> QueryResult<Todo> {
    let query = QUERY_MAP.get("get_todo_by_id").expect("Missing query");
    sql_query(query)
        .bind::<diesel::sql_types::Integer, _>(id)
        .get_result(conn)
}
pub fn update_todo(conn: &mut SqliteConnection, id: i32, updated: NewTodo) -> QueryResult<usize> {
    let query = QUERY_MAP.get("update_todo").expect("Missing query");
    sql_query(query)
        .bind::<diesel::sql_types::Integer, _>(id)
        .bind::<Text, _>(&updated.title)
        .bind::<Bool, _>(updated.completed)
        .execute(conn)
}
pub fn delete_todo(conn: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
    let query = QUERY_MAP.get("delete_todo").expect("Missing query");
    sql_query(query)
        .bind::<diesel::sql_types::Integer, _>(id)
        .execute(conn)
}
pub fn get_completed_todos(conn: &mut SqliteConnection) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP.get("get_completed_todos").expect("Missing query");
    sql_query(query).load::<Todo>(conn)
}
pub fn get_pending_todos(conn: &mut SqliteConnection) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP.get("get_pending_todos").expect("Missing query");
    sql_query(query).load::<Todo>(conn)
}
pub fn get_todos_by_title(conn: &mut SqliteConnection, title: &str) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP.get("get_todos_by_title").expect("Missing query");
    sql_query(query).bind::<Text, _>(title).load::<Todo>(conn)
}
pub fn get_todos_by_completion_status(
    conn: &mut SqliteConnection,
    completed: bool,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_by_completion_status")
        .expect("Missing query");
    sql_query(query)
        .bind::<Bool, _>(completed)
        .load::<Todo>(conn)
}
pub fn get_todos_by_date_range(
    conn: &mut SqliteConnection,
    start_date: &str,
    end_date: &str,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_by_date_range")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(start_date)
        .bind::<Text, _>(end_date)
        .load::<Todo>(conn)
}

pub fn get_todos_by_id_list(conn: &mut SqliteConnection, ids: &[i32]) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_by_id_list")
        .expect("Missing query");
    let mut query_builder = sql_query(query);

    for id in ids {
        query_builder = query_builder.bind::<diesel::sql_types::Integer, _>(*id);
    }

    query_builder.load::<Todo>(conn)
}
pub fn get_todos_by_title_and_completion(
    conn: &mut SqliteConnection,
    title: &str,
    completed: bool,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_by_title_and_completion")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(title)
        .bind::<Bool, _>(completed)
        .load::<Todo>(conn)
}
pub fn get_todos_by_title_or_completion(
    conn: &mut SqliteConnection,
    title: &str,
    completed: bool,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_by_title_or_completion")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(title)
        .bind::<Bool, _>(completed)
        .load::<Todo>(conn)
}
pub fn get_todos_by_title_and_date_range(
    conn: &mut SqliteConnection,
    title: &str,
    start_date: &str,
    end_date: &str,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_by_title_and_date_range")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(title)
        .bind::<Text, _>(start_date)
        .bind::<Text, _>(end_date)
        .load::<Todo>(conn)
}
pub fn get_todos_by_completion_and_date_range(
    conn: &mut SqliteConnection,
    completed: bool,
    start_date: &str,
    end_date: &str,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_by_completion_and_date_range")
        .expect("Missing query");
    sql_query(query)
        .bind::<Bool, _>(completed)
        .bind::<Text, _>(start_date)
        .bind::<Text, _>(end_date)
        .load::<Todo>(conn)
}
pub fn get_todos_by_title_completion_and_date_range(
    conn: &mut SqliteConnection,
    title: &str,
    completed: bool,
    start_date: &str,
    end_date: &str,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_by_title_completion_and_date_range")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(title)
        .bind::<Bool, _>(completed)
        .bind::<Text, _>(start_date)
        .bind::<Text, _>(end_date)
        .load::<Todo>(conn)
}
pub fn get_todos_by_title_or_completion_and_date_range(
    conn: &mut SqliteConnection,
    title: &str,
    completed: bool,
    start_date: &str,
    end_date: &str,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_by_title_or_completion_and_date_range")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(title)
        .bind::<Bool, _>(completed)
        .bind::<Text, _>(start_date)
        .bind::<Text, _>(end_date)
        .load::<Todo>(conn)
}
pub fn get_todos_by_multiple_conditions(
    conn: &mut SqliteConnection,
    title: Option<&str>,
    completed: Option<bool>,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_by_multiple_conditions")
        .expect("Missing query");
    let mut query_builder = sql_query(query);

    if let Some(t) = title {
        query_builder = query_builder.bind::<Text, _>(t);
    }
    if let Some(c) = completed {
        query_builder = query_builder.bind::<Bool, _>(c);
    }
    if let Some(sd) = start_date {
        query_builder = query_builder.bind::<Text, _>(sd);
    }
    if let Some(ed) = end_date {
        query_builder = query_builder.bind::<Text, _>(ed);
    }

    query_builder.load::<Todo>(conn)
}
pub fn get_todos_with_pagination(
    conn: &mut SqliteConnection,
    limit: i64,
    offset: i64,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_with_pagination")
        .expect("Missing query");
    sql_query(query)
        .bind::<diesel::sql_types::BigInt, _>(limit)
        .bind::<diesel::sql_types::BigInt, _>(offset)
        .load::<Todo>(conn)
}
pub fn count_todos(conn: &mut SqliteConnection) -> QueryResult<i64> {
    let query = QUERY_MAP.get("count_todos").expect("Missing query");
    sql_query(query).get_result(conn)
}
pub fn count_completed_todos(conn: &mut SqliteConnection) -> QueryResult<i64> {
    let query = QUERY_MAP
        .get("count_completed_todos")
        .expect("Missing query");
    sql_query(query).get_result(conn)
}
pub fn count_pending_todos(conn: &mut SqliteConnection) -> QueryResult<i64> {
    let query = QUERY_MAP.get("count_pending_todos").expect("Missing query");
    sql_query(query).get_result(conn)
}
pub fn count_todos_by_title(conn: &mut SqliteConnection, title: &str) -> QueryResult<i64> {
    let query = QUERY_MAP
        .get("count_todos_by_title")
        .expect("Missing query");
    sql_query(query).bind::<Text, _>(title).get_result(conn)
}
pub fn count_todos_by_completion_status(
    conn: &mut SqliteConnection,
    completed: bool,
) -> QueryResult<i64> {
    let query = QUERY_MAP
        .get("count_todos_by_completion_status")
        .expect("Missing query");
    sql_query(query).bind::<Bool, _>(completed).get_result(conn)
}
pub fn count_todos_by_date_range(
    conn: &mut SqliteConnection,
    start_date: &str,
    end_date: &str,
) -> QueryResult<i64> {
    let query = QUERY_MAP
        .get("count_todos_by_date_range")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(start_date)
        .bind::<Text, _>(end_date)
        .get_result(conn)
}
pub fn count_todos_by_id_list(conn: &mut SqliteConnection, ids: &[i32]) -> QueryResult<i64> {
    let query = QUERY_MAP
        .get("count_todos_by_id_list")
        .expect("Missing query");
    let mut query_builder = sql_query(query);

    for id in ids {
        query_builder = query_builder.bind::<diesel::sql_types::Integer, _>(*id);
    }

    query_builder.get_result(conn)
}
pub fn count_todos_by_title_and_completion(
    conn: &mut SqliteConnection,
    title: &str,
    completed: bool,
) -> QueryResult<i64> {
    let query = QUERY_MAP
        .get("count_todos_by_title_and_completion")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(title)
        .bind::<Bool, _>(completed)
        .get_result(conn)
}
pub fn count_todos_by_title_or_completion(
    conn: &mut SqliteConnection,
    title: &str,
    completed: bool,
) -> QueryResult<i64> {
    let query = QUERY_MAP
        .get("count_todos_by_title_or_completion")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(title)
        .bind::<Bool, _>(completed)
        .get_result(conn)
}
pub fn count_todos_by_title_and_date_range(
    conn: &mut SqliteConnection,
    title: &str,
    start_date: &str,
    end_date: &str,
) -> QueryResult<i64> {
    let query = QUERY_MAP
        .get("count_todos_by_title_and_date_range")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(title)
        .bind::<Text, _>(start_date)
        .bind::<Text, _>(end_date)
        .get_result(conn)
}
pub fn count_todos_by_completion_and_date_range(
    conn: &mut SqliteConnection,
    completed: bool,
    start_date: &str,
    end_date: &str,
) -> QueryResult<i64> {
    let query = QUERY_MAP
        .get("count_todos_by_completion_and_date_range")
        .expect("Missing query");
    sql_query(query)
        .bind::<Bool, _>(completed)
        .bind::<Text, _>(start_date)
        .bind::<Text, _>(end_date)
        .get_result(conn)
}
pub fn count_todos_by_title_completion_and_date_range(
    conn: &mut SqliteConnection,
    title: &str,
    completed: bool,
    start_date: &str,
    end_date: &str,
) -> QueryResult<i64> {
    let query = QUERY_MAP
        .get("count_todos_by_title_completion_and_date_range")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(title)
        .bind::<Bool, _>(completed)
        .bind::<Text, _>(start_date)
        .bind::<Text, _>(end_date)
        .get_result(conn)
}
pub fn count_todos_by_title_or_completion_and_date_range(
    conn: &mut SqliteConnection,
    title: &str,
    completed: bool,
    start_date: &str,
    end_date: &str,
) -> QueryResult<i64> {
    let query = QUERY_MAP
        .get("count_todos_by_title_or_completion_and_date_range")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(title)
        .bind::<Bool, _>(completed)
        .bind::<Text, _>(start_date)
        .bind::<Text, _>(end_date)
        .get_result(conn)
}
pub fn count_todos_by_multiple_conditions(
    conn: &mut SqliteConnection,
    title: Option<&str>,
    completed: Option<bool>,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> QueryResult<i64> {
    let query = QUERY_MAP
        .get("count_todos_by_multiple_conditions")
        .expect("Missing query");
    let mut query_builder = sql_query(query);

    if let Some(t) = title {
        query_builder = query_builder.bind::<Text, _>(t);
    }
    if let Some(c) = completed {
        query_builder = query_builder.bind::<Bool, _>(c);
    }
    if let Some(sd) = start_date {
        query_builder = query_builder.bind::<Text, _>(sd);
    }
    if let Some(ed) = end_date {
        query_builder = query_builder.bind::<Text, _>(ed);
    }

    query_builder.get_result(conn)
}
pub fn get_todos_with_sorting(
    conn: &mut SqliteConnection,
    sort_by: &str,
    sort_order: &str,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_with_sorting")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(sort_by)
        .bind::<Text, _>(sort_order)
        .load::<Todo>(conn)
}
pub fn get_todos_with_filtering(
    conn: &mut SqliteConnection,
    filter_by: &str,
    filter_value: &str,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_with_filtering")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(filter_by)
        .bind::<Text, _>(filter_value)
        .load::<Todo>(conn)
}
pub fn get_todos_with_sorting_and_filtering(
    conn: &mut SqliteConnection,
    sort_by: &str,
    sort_order: &str,
    filter_by: &str,
    filter_value: &str,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_with_sorting_and_filtering")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(sort_by)
        .bind::<Text, _>(sort_order)
        .bind::<Text, _>(filter_by)
        .bind::<Text, _>(filter_value)
        .load::<Todo>(conn)
}
pub fn get_todos_with_aggregation(
    conn: &mut SqliteConnection,
    aggregation_type: &str,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_with_aggregation")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(aggregation_type)
        .load::<Todo>(conn)
}
pub fn get_todos_with_aggregation_and_filtering(
    conn: &mut SqliteConnection,
    aggregation_type: &str,
    filter_by: &str,
    filter_value: &str,
) -> QueryResult<Vec<Todo>> {
    let query = QUERY_MAP
        .get("get_todos_with_aggregation_and_filtering")
        .expect("Missing query");
    sql_query(query)
        .bind::<Text, _>(aggregation_type)
        .bind::<Text, _>(filter_by)
        .bind::<Text, _>(filter_value)
        .load::<Todo>(conn)
}
