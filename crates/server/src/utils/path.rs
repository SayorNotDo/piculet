use std::path::Path;
use axum::extract::Query;
use serde::Deserialize;

pub type PathQuery = Query<Path>;

#[derive(Debug, Clone, Deserialize)]
pub struct Path {
    pub path: String,
}

pub fn validate_path(path: PathQuery) -> Result<String> {
    let path = path.path.clone();

    if path.contains("..") {
        return Err("Path cannot contain '..'".to_string())
    }

    if path.contains('~') {
        return Err("Path cannot contain '~'".to_string())
    }

    Ok(path)
}