use serde_json::Value;

/// Represents the type of SQL operation to perform.
#[derive(Debug)]
pub enum Operation {
    Select,
    Insert,
    Update,
    Delete,
}

/// Holds information used to construct a SQL query.
#[derive(Debug)]
pub struct QueryParams {
    pub table_name: String,
    pub operation: Operation,
    pub conditions: Value,
    pub data: Value,
}
