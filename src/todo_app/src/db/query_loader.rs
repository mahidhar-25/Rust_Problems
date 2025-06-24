use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, fs};

pub static QUERY_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let content = fs::read_to_string("queries.sql").expect("Failed to read queries.sql file");

    let mut map = HashMap::new();
    let re = Regex::new(r"-- name: (\w+)\n(.*?)(?=(-- name:|\z))").unwrap();

    for cap in re.captures_iter(&content) {
        let name = cap[1].trim().to_string();
        let sql = cap[2].trim().to_string();
        map.insert(name, sql);
    }

    map
});
