use std::collections::HashMap;
use std::fs;

pub fn load_queries(path: &str) -> HashMap<String, String> {
    let contents = fs::read_to_string(path).expect("Failed to read SQL file");

    let mut queries = HashMap::new();
    let mut current_name = None;
    let mut current_sql = String::new();

    for line in contents.lines() {
        if let Some(name) = line.strip_prefix("-- name: ") {
            if let Some(name) = current_name.take() {
                queries.insert(name, current_sql.trim().to_string());
                current_sql = String::new();
            }
            current_name = Some(name.trim().to_string());
        } else if current_name.is_some() {
            current_sql.push_str(line);
            current_sql.push('\n');
        }
    }

    if let Some(name) = current_name {
        queries.insert(name, current_sql.trim().to_string());
    }

    queries
}
