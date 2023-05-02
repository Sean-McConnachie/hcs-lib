pub async fn initialize_db(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS change_types (
            id SMALLSERIAL PRIMARY KEY,
            description VARCHAR(32) NOT NULL
        );
    "#;
    sqlx::query(sql).execute(pool).await?;

    let sql = r#"
        CREATE TABLE IF NOT EXISTS change_events (
            id SERIAL PRIMARY KEY,
            change_type_id SMALLINT NOT NULL REFERENCES change_types(id),
            event_time TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
    "#;
    sqlx::query(sql).execute(pool).await?;

    let sql = r#"
        INSERT INTO change_types (id, description)
        VALUES (1, 'File Create'),
               (2, 'File Modify'),
               (3, 'File Move'),
               (4, 'File Delete'),
               (5, 'Undo File Delete'),
               (6, 'Directory Create'),
               (7, 'Directory Move'),
               (8, 'Directory Delete'),
               (9, 'Undo Directory Delete'),
               (10, 'Symlink Create'),
               (11, 'Symlink Delete')
        ON CONFLICT DO NOTHING;
    "#;
    sqlx::query(sql).execute(pool).await?;

    let sql = r#"
        CREATE TABLE IF NOT EXISTS file_create (
            change_event_id INTEGER PRIMARY KEY NOT NULL REFERENCES change_events(id),
            path VARCHAR(128) NOT NULL
        )
    "#;
    sqlx::query(sql).execute(pool).await?;

    let sql = r#"
        CREATE TABLE IF NOT EXISTS file_modify (
            change_event_id INTEGER PRIMARY KEY NOT NULL REFERENCES change_events(id),
            path VARCHAR(128) NOT NULL
        )
    "#;
    sqlx::query(sql).execute(pool).await?;

    let sql = r#"
        CREATE TABLE IF NOT EXISTS file_move (
            change_event_id INTEGER PRIMARY KEY NOT NULL REFERENCES change_events(id),
            old_path VARCHAR(128) NOT NULL,
            new_path VARCHAR(128) NOT NULL
        )
    "#;
    sqlx::query(sql).execute(pool).await?;

    let sql = r#"
        CREATE TABLE IF NOT EXISTS file_delete (
            change_event_id INTEGER PRIMARY KEY NOT NULL REFERENCES change_events(id),
            path VARCHAR(128) NOT NULL
        )
    "#;
    sqlx::query(sql).execute(pool).await?;

    let sql = r#"
        CREATE TABLE IF NOT EXISTS undo_file_delete (
            change_event_id INTEGER PRIMARY KEY NOT NULL REFERENCES change_events(id),
            path VARCHAR(128) NOT NULL
        )
    "#;
    sqlx::query(sql).execute(pool).await?;

    let sql = r#"
        CREATE TABLE IF NOT EXISTS directory_create (
            change_event_id INTEGER PRIMARY KEY NOT NULL REFERENCES change_events(id),
            path VARCHAR(128) NOT NULL
        )
    "#;
    sqlx::query(sql).execute(pool).await?;

    let sql = r#"
        CREATE TABLE IF NOT EXISTS directory_move (
            change_event_id INTEGER PRIMARY KEY NOT NULL REFERENCES change_events(id),
            old_path VARCHAR(128) NOT NULL,
            new_path VARCHAR(128) NOT NULL
        )
    "#;
    sqlx::query(sql).execute(pool).await?;

    let sql = r#"
        CREATE TABLE IF NOT EXISTS directory_delete (
            change_event_id INTEGER PRIMARY KEY NOT NULL REFERENCES change_events(id),
            path VARCHAR(128) NOT NULL
        )
    "#;
    sqlx::query(sql).execute(pool).await?;

    let sql = r#"
        CREATE TABLE IF NOT EXISTS undo_directory_delete (
            change_event_id INTEGER PRIMARY KEY NOT NULL REFERENCES change_events(id),
            path VARCHAR(128) NOT NULL
        )
    "#;
    sqlx::query(sql).execute(pool).await?;

    let sql = r#"
        CREATE TABLE IF NOT EXISTS symlink_create (
            change_event_id INTEGER PRIMARY KEY NOT NULL REFERENCES change_events(id),
            path VARCHAR(128) NOT NULL,
            target VARCHAR(128) NOT NULL
        )
    "#;
    sqlx::query(sql).execute(pool).await?;

    let sql = r#"
        CREATE TABLE IF NOT EXISTS symlink_delete (
            change_event_id INTEGER PRIMARY KEY NOT NULL REFERENCES change_events(id),
            path VARCHAR(128) NOT NULL
        )
    "#;
    sqlx::query(sql).execute(pool).await?;

    Ok(())
}
