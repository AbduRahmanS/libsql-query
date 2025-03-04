use std::env;

use crate::query_params::QueryParams;
use crate::table::Table;
use crate::utils::construct_statement;
use dotenvy::dotenv;
use libsql::{Builder, Connection, Result as QueryResult, Rows, Transaction};

/// Provides a connection to the database and manages transactions.
pub struct Client {
    pub conn: Connection,
    pub tx: Option<Transaction>,
}

impl Client {
    /// Creates a new `Client` instance, connecting to the configured database.
    pub async fn new() -> Self {
        dotenv().expect("Failed to load .env file");
        let db_path = env::var("DB_PATH").expect("DB_PATH not set in .env");

        let conn = Builder::new_local(db_path)
            .build()
            .await
            .expect("Failed to build database connection")
            .connect()
            .expect("Failed to Connect to Database");
        Client { conn, tx: None }
    }

    /// Begins a database transaction.
    pub async fn begin_transaction(&mut self) -> Result<(), libsql::Error> {
        if self.tx.is_some() {
            return Err(libsql::Error::SqliteFailure(
                5,
                "Transaction already in progress".to_string(),
            ));
        }
        let tx = Some(
            self.conn
                .transaction_with_behavior(libsql::TransactionBehavior::Immediate)
                .await?,
        );
        self.tx = tx;
        println!("Begin Transaction");
        Ok(())
    }

    /// Commits the open transaction, if any.
    pub async fn commit(self) -> Result<(), libsql::Error> {
        match self.tx {
            Some(tx) => tx.commit().await,
            None => Ok(()),
        }
    }

    /// Rolls back the current transaction, if any exists.
    pub async fn rollback(&mut self) -> Result<(), libsql::Error> {
        if let Some(tx) = self.tx.take() {
            println!("Rollback Transaction");
            tx.rollback().await
        } else {
            Ok(())
        }
    }

    /// Returns a `Table` instance for the specified table name.
    pub fn table(&self, table_name: &str) -> Table {
        Table {
            client: self,
            table_name: table_name.to_string(),
        }
    }

    /// Executes a SQL query built from the provided `QueryParams`.
    pub async fn query(&self, params: QueryParams) -> QueryResult<Rows> {
        let statement = construct_statement(params);
        match &self.tx {
            Some(tx) => {
                println!("Querying with Transaction");
                tx.query(&statement.0, statement.1).await
            }
            None => {
                println!("Querying without Transaction");
                self.conn.query(&statement.0, statement.1).await
            }
        }
    }
}
