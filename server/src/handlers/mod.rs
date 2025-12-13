use serde::Serialize;
use serde_json::Value;

pub mod cabinet;
pub mod direction;
pub mod journal;
pub mod laborant;
pub mod student;
pub mod summary;

pub fn json_ok<T: Serialize>(data: T) -> Value {
    serde_json::json!({ "success": true, "data": data })
}

pub fn json_err(msg: &str) -> Value {
    serde_json::json!({ "success": false, "message": msg })
}
