pub async fn get_server_version(db_pool: &sqlx::PgPool) -> Result<i32, sqlx::Error> {
    let version = sqlx::query!("select max(id) from change_events;")
        .fetch_one(db_pool)
        .await?;
    let version = version.max.unwrap_or(0 as i32);

    Ok(version)
}
