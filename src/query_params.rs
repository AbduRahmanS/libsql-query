use serde_json::Value;

#[derive(Debug)]
pub enum Operation {
    Select,
    Insert,
    Update,
    Delete,
}

#[derive(Debug)]
pub struct QueryParams {
    pub table_name: String,
    pub operation: Operation,
    pub conditions: Value,
    pub data: Value,
}
