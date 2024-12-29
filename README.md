# libsql-query

`libsql-query` is a Rust library that provides a simple interface for interacting with SQLite databases using the [`libsql`](https://crates.io/crates/libsql) crate. It simplifies common database operations like `SELECT`, `INSERT`, `UPDATE`, and `DELETE` by providing high-level abstractions and automatic parameter handling.

## Features

- **Easy-to-use API**: Simplifies database interactions with straightforward methods.
- **JSON Integration**: Utilizes `serde_json` for seamless JSON data handling.
- **Transaction Support**: Provides methods for transaction management.
- **Parameterized Queries**: Protects against SQL injection with parameterized queries.

## Installation

Add `libsql-query` to your `Cargo.toml` dependencies:

```toml
[dependencies]
libsql-query = "0.1.0"
```

## Usage

#### 1. Add the `DB_PATH` to your environment variables.

#### 2. Create a libsql-query client instance.

```rust
use libsql_query::client::Client;
let client = Client::new().await
```

#### 3. Insert data into a table.

```rust
let data = json!({
    "name": "Alice",
    "age": 30,
    "email": "alice@company.com"
});
let rows = client.table("users").insert(data).await?;
// rows: libsql::Rows
```

#### 4. Retrieve data from a table.

```rust
let rows = client.table("users").select(json!({"id": 1})).await?;
// rows: libsql::Rows
```

#### 5. Transactions

```rust
let mut client = Client::new().await; // Client should be mutable
client.begin_transaction().await;
let result = async {
    // Perform operations like usualy
    client.table("users").update(data).await?;
    client.table("users").delete().await?;
};

if result.is_ok() {
    client.commit().await;
} else {
    client.rollback().await;
}
```

## Roadmap

> **Note:** No particular order and subject to change.

- [x] **Improve Documentation**: Enhance documentation with more examples and detailed explanations.
- [ ] **Testing**: Implement comprehensive tests to ensure the library's correctness and reliability.
- [ ] **Error Handling**: Enhance error handling to cover more edge cases and provide better diagnostics.
- [ ] **Advanced Queries**: Support advanced queries like joins, subqueries, and aggregations.
- [ ] **Filtering**: Implement flexible filtering mechanisms for more precise data retrieval.
- [ ] **Type-Safe Columns**: Introduce type-safe representations of table columns for better compile-time checks and safety.
- [ ] **Performance Optimization**: Optimize the library for better performance and efficiency.
