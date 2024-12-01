use crate::query_params::QueryParams;
use crate::table::Table;
use crate::utils::construct_statement;
use libsql::{Builder, Connection, Result as QueryResult, Rows, Transaction};

pub struct Client {
    pub conn: Connection,
    pub tx: Option<Transaction>,
}

impl Client {
    pub async fn new(db_path: String) -> Self {
        let conn = Builder::new_local(db_path)
            .build()
            .await
            .expect("Failed to build database connection")
            .connect()
            .expect("Failed to Connect to Database");
        Client { conn, tx: None }
    }

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

    pub async fn commit(self) -> Result<(), libsql::Error> {
        match self.tx {
            Some(tx) => tx.commit().await,
            None => Ok(()),
        }
    }

    pub async fn rollback(&mut self) -> Result<(), libsql::Error> {
        if let Some(tx) = self.tx.take() {
            println!("Rollback Transaction");
            tx.rollback().await
        } else {
            Ok(())
        }
    }

    pub fn table(&self, table_name: &str) -> Table {
        Table {
            client: self,
            table_name: table_name.to_string(),
        }
    }

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
