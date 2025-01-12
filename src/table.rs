use crate::client::Client;
use crate::query_params::{Operation, QueryParams};
use libsql::{Result as QueryResult, Rows};
use serde_json::Value;

/// Represents a database table and provides methods for basic CRUD operations.
pub struct Table<'a> {
    pub client: &'a Client,
    pub table_name: String,
}

impl<'a> Table<'a> {
    /// Retrieves rows from the table based on the given JSON conditions.
    pub async fn select(&self, conditions: Value) -> QueryResult<Rows> {
        let params = QueryParams {
            table_name: self.table_name.clone(),
            operation: Operation::Select,
            conditions,
            data: Value::Null,
        };
        self.client.query(params).await
    }

    /// Inserts rows using the provided JSON data and returns inserted rows.
    pub async fn insert(&self, data: Value) -> QueryResult<Rows> {
        let params = QueryParams {
            table_name: self.table_name.clone(),
            operation: Operation::Insert,
            conditions: Value::Null,
            data,
        };
        self.client.query(params).await
    }

    /// Updates rows matching the JSON conditions with the given JSON data.
    pub async fn update(&self, conditions: Value, data: Value) -> QueryResult<Rows> {
        let params = QueryParams {
            table_name: self.table_name.clone(),
            operation: Operation::Update,
            conditions,
            data,
        };
        self.client.query(params).await
    }

    /// Deletes rows from the table matching the provided JSON conditions.
    pub async fn delete(&self, conditions: Value) -> QueryResult<Rows> {
        let params = QueryParams {
            table_name: self.table_name.clone(),
            operation: Operation::Delete,
            conditions,
            data: Value::Null,
        };
        self.client.query(params).await
    }
}
