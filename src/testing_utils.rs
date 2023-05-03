#![allow(unused)]
use std::{env, fs};

use crate::{
    client_database,
    server_database::{connect_db, DbConfig},
};

pub fn rm_dirs_ce_dirs_get_default_helpers() -> (
    client_database::FileHandlerConfig,
    client_database::ChangeCounter,
) {
    let current_working_directory = env::current_dir().unwrap();

    let file_handler_config = client_database::FileHandlerConfig::new(
        current_working_directory
            .join("_storage_dir".to_string())
            .to_str()
            .unwrap()
            .to_string(),
        current_working_directory
            .join("_symlink_dir".to_string())
            .to_str()
            .unwrap()
            .to_string(),
        current_working_directory
            .join("_temporary_dir".to_string())
            .to_str()
            .unwrap()
            .to_string(),
    );

    // remove all directories and create them again
    fs::remove_dir_all(&file_handler_config.storage_directory);
    fs::remove_dir_all(&file_handler_config.symlink_directory);
    fs::remove_dir_all(&file_handler_config.temporary_directory);
    fs::remove_dir_all(&file_handler_config.program_data_directory);
    fs::create_dir_all(&file_handler_config.storage_directory);
    fs::create_dir_all(&file_handler_config.symlink_directory);
    fs::create_dir_all(&file_handler_config.temporary_directory);
    fs::create_dir_all(&file_handler_config.program_data_directory);

    let mut change_counter =
        client_database::ChangeCounter::init(&file_handler_config.program_data_directory);
    (file_handler_config, change_counter)
}
pub async fn clear_tables_and_get_pool() -> Result<sqlx::postgres::PgPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

    let db_conf = DbConfig::new(db_url, 5);

    let db_pool = connect_db(&db_conf).await;
    assert!(db_pool.is_ok());
    let db_pool = db_pool.unwrap();

    let sql = r#"TRUNCATE TABLE change_events CASCADE;"#;
    let result = sqlx::query(sql).execute(&db_pool).await;

    assert!(result.is_ok());

    Ok(db_pool)
}
