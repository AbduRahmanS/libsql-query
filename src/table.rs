use crate::client::Client;
use crate::query_params::{Operation, QueryParams};
use libsql::{Result as QueryResult, Rows};
use serde_json::Value;

pub struct Table<'a> {
    pub client: &'a Client,
    pub table_name: String,
}

impl<'a> Table<'a> {
    pub async fn select(&self, conditions: Value) -> QueryResult<Rows> {
        let params = QueryParams {
            table_name: self.table_name.clone(),
            operation: Operation::Select,
            conditions,
            data: Value::Null,
        };
        self.client.query(params).await
    }

    pub async fn insert(&self, data: Value) -> QueryResult<Rows> {
        let params = QueryParams {
            table_name: self.table_name.clone(),
            operation: Operation::Insert,
            conditions: Value::Null,
            data: data,
        };
        self.client.query(params).await
    }

    pub async fn update(&self, conditions: Value, data: Value) -> QueryResult<Rows> {
        let params = QueryParams {
            table_name: self.table_name.clone(),
            operation: Operation::Update,
            conditions,
            data,
        };
        self.client.query(params).await
    }

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
