# libsql-query

`libsql-query` is a Rust library that provides a simple interface for interacting with SQLite databases using the [`libsql`](https://crates.io/crates/libsql) crate. It simplifies common database operations like `SELECT`, `INSERT`, `UPDATE`, and `DELETE` by providing high-level abstractions and automatic parameter handling.

## Features

- **Easy-to-use API**: Simplifies database interactions with straightforward methods.
- **JSON Integration**: Utilizes `serde_json::Value` for seamless JSON data handling.
- **Transaction Support**: Provides methods for transaction management.
- **Parameterized Queries**: Protects against SQL injection with parameterized queries.

## Installation

Add `libsql-query` to your `Cargo.toml` dependencies:

## Roadmap

- **Improve Documentation**: Enhance documentation with more examples and detailed explanations.
- **Testing**: Implement comprehensive tests to ensure the library's correctness and reliability.
- **Error Handling**: Enhance error handling to cover more edge cases and provide better diagnostics.
- **Advanced Queries**: Support advanced queries like joins, subqueries, and aggregations.
- **Filtering**: Implement flexible filtering mechanisms for more precise data retrieval.
- **Type-Safe Columns**: Introduce type-safe representations of table columns for better compile-time checks and safety.
- **Performance Optimization**: Optimize the library for better performance and efficiency.
