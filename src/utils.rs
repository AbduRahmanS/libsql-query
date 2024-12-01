use crate::query_params::{Operation, QueryParams};
use libsql::Value as SqlValue;
use serde_json::Value;

pub fn json_to_sql_value(json_value: &Value) -> SqlValue {
    match json_value {
        Value::String(s) => SqlValue::from(s.clone()),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                SqlValue::from(i)
            } else if let Some(f) = n.as_f64() {
                SqlValue::from(f)
            } else {
                panic!("Unsupported number type")
            }
        }
        Value::Bool(b) => SqlValue::from(*b),
        Value::Null => SqlValue::Null,
        Value::Array(_) => todo!(),
        Value::Object(_) => todo!(),
    }
}

pub fn construct_statement(params: QueryParams) -> (String, Vec<SqlValue>) {
    let mut query = String::new();
    let mut query_params: Vec<SqlValue> = Vec::new();

    match params.operation {
        Operation::Select => {
            query.push_str(&format!("SELECT * FROM {}", params.table_name));
            if params.conditions != Value::Null {
                let conditions = params.conditions.as_object().unwrap();
                let condition_str = conditions
                    .iter()
                    .map(|(k, _v)| format!("{} = ?", k))
                    .collect::<Vec<String>>()
                    .join(" AND ");
                query.push_str(&format!(" WHERE {};", condition_str));
                query_params.extend(conditions.values().map(|v| json_to_sql_value(v)));
            }
        }
        Operation::Insert => {
            let data = params.data.as_object().unwrap();
            let columns = data.keys().cloned().collect::<Vec<String>>().join(", ");
            let placeholders = vec!["?"; data.len()].join(", ");
            query.push_str(&format!(
                "INSERT INTO {} ({}) VALUES ({}) RETURNING *;",
                params.table_name, columns, placeholders
            ));
            query_params.extend(data.values().map(|v| json_to_sql_value(v)));
        }
        Operation::Update => {
            let data = params.data.as_object().unwrap();
            let update_str = data
                .iter()
                .map(|(k, _v)| format!("{} = ?", k))
                .collect::<Vec<String>>()
                .join(", ");
            query.push_str(&format!("UPDATE {} SET {}", params.table_name, update_str));
            query_params.extend(data.values().map(|v| json_to_sql_value(v)));
            let conditions = params.conditions.as_object().unwrap();
            let condition_str = conditions
                .iter()
                .map(|(k, _v)| format!("{} = ?", k))
                .collect::<Vec<String>>()
                .join(" AND ");
            query.push_str(&format!(" WHERE {};", condition_str));
            query_params.extend(conditions.values().map(|v| json_to_sql_value(v)));
        }
        Operation::Delete => {
            query.push_str(&format!("DELETE FROM {}", params.table_name));
            let conditions = params.conditions.as_object().unwrap();

            let condition_str = conditions
                .iter()
                .map(|(k, _v)| format!("{} = ?", k))
                .collect::<Vec<String>>()
                .join(" AND ");
            query.push_str(&format!(" WHERE {};", condition_str));
            query_params.extend(conditions.values().map(|v| json_to_sql_value(v)));
        }
    }
    (query, query_params)
}
