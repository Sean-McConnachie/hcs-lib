mod get_changes;
mod initialize;
mod insert_change;
mod table_details;

pub use get_changes::get_changes;
pub use initialize::initialize_db;
pub use insert_change::insert_change;
pub use table_details::{TableDetails, TableDetailsTrait, TABLES};

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::ConnectOptions;
use std::str::FromStr;

pub struct DbConfig {
    pub database_url: String,
    pub max_connections: u32,
}

pub async fn connect_db(db_conf: &DbConfig) -> Result<sqlx::PgPool, sqlx::Error> {
    let options = PgConnectOptions::from_str(db_conf.database_url.as_str())?
        .disable_statement_logging()
        .clone();
    let pool = PgPoolOptions::new()
        .max_connections(db_conf.max_connections)
        .connect_with(options)
        .await?;

    Ok(pool)
}

pub async fn execute_query(db_pool: &sqlx::PgPool, sql: &str) -> Result<(), sqlx::Error> {
    sqlx::query(sql).execute(db_pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // DATABASE_URL=postgres://postgres:12341234@localhost/hcs_testing cargo test
    use super::{connect_db, execute_query, DbConfig};
    use std::env;

    #[tokio::test]
    async fn test_connect_db() {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

        let db_conf = DbConfig {
            database_url: db_url,
            max_connections: 5,
        };

        let db_pool = connect_db(&db_conf).await;
        assert!(db_pool.is_ok());
        let db_pool = db_pool.unwrap();

        let query = "SELECT 1";
        let result = execute_query(&db_pool, query).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_tables() {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

        let db_conf = DbConfig {
            database_url: db_url,
            max_connections: 5,
        };

        let db_pool = connect_db(&db_conf).await;
        assert!(db_pool.is_ok());
        let db_pool = db_pool.unwrap();

        super::initialize_db(&db_pool).await.unwrap();
    }
}
