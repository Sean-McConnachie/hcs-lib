use sqlx::Row;

use crate::data;

use super::TableDetailsTrait;

pub async fn insert_change(
    change: data::ChangeEvent,
    db_pool: &sqlx::PgPool,
) -> Result<i32, sqlx::Error> {
    // begin transaction
    let mut transaction = db_pool.begin().await?;

    // insert change
    let change_id: i32 = sqlx::query(
        r#"
        INSERT INTO change_events (change_type_id)
        VALUES ($1)
        RETURNING id
        "#,
    )
    .bind(change.table_details().change_type_id())
    .map(|row: sqlx::postgres::PgRow| row.get(0))
    .fetch_one(&mut transaction)
    .await?;

    match change {
        data::ChangeEvent::File(file_event) => match file_event {
            data::FileEvent::Create(file_create) => {
                sqlx::query(
                    r#"
                        INSERT INTO file_create (change_event_id, path)
                        VALUES ($1, $2)
                        "#,
                )
                .bind(change_id)
                .bind(file_create.path())
                .execute(&mut transaction)
                .await?;
            }
            data::FileEvent::Delete(file_delete) => {
                sqlx::query(
                    r#"
                        INSERT INTO file_delete (change_event_id, path)
                        VALUES ($1, $2)
                        "#,
                )
                .bind(change_id)
                .bind(file_delete.path())
                .execute(&mut transaction)
                .await?;
            }
            data::FileEvent::Modify(file_modify) => {
                sqlx::query(
                    r#"
                        INSERT INTO file_modify (change_event_id, path)
                        VALUES ($1, $2)
                        "#,
                )
                .bind(change_id)
                .bind(file_modify.path())
                .execute(&mut transaction)
                .await?;
            }
            data::FileEvent::Move(file_move) => {
                sqlx::query(
                    r#"
                        INSERT INTO file_move (change_event_id, old_path, new_path)
                        VALUES ($1, $2, $3)
                        "#,
                )
                .bind(change_id)
                .bind(file_move.from_path())
                .bind(file_move.to_path())
                .execute(&mut transaction)
                .await?;
            }
            data::FileEvent::UndoDelete(file_undo_delete) => {
                sqlx::query(
                    r#"
                        INSERT INTO file_undo_delete (change_event_id, path)
                        VALUES ($1, $2)
                        "#,
                )
                .bind(change_id)
                .bind(file_undo_delete.path())
                .execute(&mut transaction)
                .await?;
            }
        },
        data::ChangeEvent::Directory(directory_event) => match directory_event {
            data::DirectoryEvent::Create(directory_create) => {
                sqlx::query(
                    r#"
                        INSERT INTO directory_create (change_event_id, path)
                        VALUES ($1, $2)
                        "#,
                )
                .bind(change_id)
                .bind(directory_create.path())
                .execute(&mut transaction)
                .await?;
            }
            data::DirectoryEvent::Delete(directory_delete) => {
                sqlx::query(
                    r#"
                        INSERT INTO directory_delete (change_event_id, path)
                        VALUES ($1, $2)
                        "#,
                )
                .bind(change_id)
                .bind(directory_delete.path())
                .execute(&mut transaction)
                .await?;
            }
            data::DirectoryEvent::Move(directory_move) => {
                sqlx::query(
                    r#"
                        INSERT INTO directory_move (change_event_id, old_path, new_path)
                        VALUES ($1, $2, $3)
                        "#,
                )
                .bind(change_id)
                .bind(directory_move.from_path())
                .bind(directory_move.to_path())
                .execute(&mut transaction)
                .await?;
            }
            data::DirectoryEvent::UndoDelete(directory_undo_delete) => {
                sqlx::query(
                    r#"
                        INSERT INTO directory_undo_delete (change_event_id, path)
                        VALUES ($1, $2)
                        "#,
                )
                .bind(change_id)
                .bind(directory_undo_delete.path())
                .execute(&mut transaction)
                .await?;
            }
        },
        data::ChangeEvent::Symlink(symlink_event) => match symlink_event {
            data::SymlinkEvent::Create(symlink_create) => {
                sqlx::query(
                    r#"
                        INSERT INTO symlink_create (change_event_id, path, target)
                        VALUES ($1, $2, $3)
                        "#,
                )
                .bind(change_id)
                .bind(symlink_create.path())
                .bind(symlink_create.links_to())
                .execute(&mut transaction)
                .await?;
            }
            data::SymlinkEvent::Delete(symlink_delete) => {
                sqlx::query(
                    r#"
                        INSERT INTO symlink_delete (change_event_id, path)
                        VALUES ($1, $2)
                        "#,
                )
                .bind(change_id)
                .bind(symlink_delete.path())
                .execute(&mut transaction)
                .await?;
            }
        },
    }

    // transaction.rollback().await?;
    transaction.commit().await?;

    Ok(change_id)
}

#[cfg(test)]
mod test {
    use super::insert_change;
    use crate::{server_database::get_changes, testing_utils::clear_tables_and_get_pool};

    #[tokio::test]
    async fn test_insert_change() {
        let db_pool = clear_tables_and_get_pool().await.unwrap();

        let change = crate::data::ChangeEvent::File(crate::data::FileEvent::Create(
            crate::data::FileCreate::new(0, "hello.txt".to_string()),
        ));

        let _change_id = insert_change(change, &db_pool).await.unwrap();

        let changes = get_changes(0, 100, &db_pool).await.unwrap();
        dbg!(&changes);
        assert_eq!(changes.len(), 1);
    }
}
