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

fn build_condition_string(conditions: &Value) -> (String, Vec<SqlValue>) {
    if conditions == &Value::Null {
        return ("".to_string(), vec![]);
    }
    let map = conditions.as_object().unwrap();
    let cond_str = map
        .keys()
        .map(|k| format!("{} = ?", k))
        .collect::<Vec<_>>()
        .join(" AND ");
    let cond_params = map.values().map(|v| json_to_sql_value(v)).collect();
    (format!(" WHERE {};", cond_str), cond_params)
}

fn build_update_string(data: &Value) -> (String, Vec<SqlValue>) {
    let map = data.as_object().unwrap();
    let filtered_map: serde_json::Map<String, Value> = map
        .iter()
        .filter(|(_, v)| !v.is_null()) // Filter out null values
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let update_str = filtered_map
        .keys()
        .map(|k| format!("{} = ?", k))
        .collect::<Vec<_>>()
        .join(", ");
    let update_params = filtered_map
        .values()
        .map(|v| json_to_sql_value(v))
        .collect();
    (update_str, update_params)
}

pub fn construct_statement(params: QueryParams) -> (String, Vec<SqlValue>) {
    let mut query = String::new();
    let mut query_params: Vec<SqlValue> = Vec::new();

    match params.operation {
        Operation::Select => {
            query.push_str(&format!("SELECT * FROM {}", params.table_name));
            let (cond_str, cond_params) = build_condition_string(&params.conditions);
            if !cond_str.is_empty() {
                query.push_str(&cond_str);
                query_params.extend(cond_params);
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
            let (update_str, update_params) = build_update_string(&params.data);
            query.push_str(&format!("UPDATE {} SET {}", params.table_name, update_str));
            query_params.extend(update_params);
            let (cond_str, cond_params) = build_condition_string(&params.conditions);
            query.push_str(&cond_str);
            query_params.extend(cond_params);
        }
        Operation::Delete => {
            query.push_str(&format!("DELETE FROM {}", params.table_name));
            let (cond_str, cond_params) = build_condition_string(&params.conditions);
            query.push_str(&cond_str);
            query_params.extend(cond_params);
        }
    }
    (query, query_params)
}
