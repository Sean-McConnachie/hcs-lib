use sqlx::Row;

use crate::data;

fn row_to_change_event(
    row: sqlx::postgres::PgRow,
    table: &super::TableDetails,
) -> (i32, data::ChangeEvent) {
    let change_event_id: i32 = row.get("change_event_id");

    match table.change_type_id() {
        1 => (
            change_event_id,
            data::ChangeEvent::File(data::FileEvent::Create(data::FileCreate::from(row).into())),
        ),
        2 => (
            change_event_id,
            data::ChangeEvent::File(data::FileEvent::Modify(data::FileModify::from(row).into())),
        ),
        3 => (
            change_event_id,
            data::ChangeEvent::File(data::FileEvent::Move(data::FileMove::from(row).into())),
        ),
        4 => (
            change_event_id,
            data::ChangeEvent::File(data::FileEvent::Delete(data::FileDelete::from(row).into())),
        ),
        5 => (
            change_event_id,
            data::ChangeEvent::File(data::FileEvent::UndoDelete(
                data::FileUndoDelete::from(row).into(),
            )),
        ),
        6 => (
            change_event_id,
            data::ChangeEvent::Directory(data::DirectoryEvent::Create(
                data::DirectoryCreate::from(row).into(),
            )),
        ),
        7 => (
            change_event_id,
            data::ChangeEvent::Directory(data::DirectoryEvent::Move(
                data::DirectoryMove::from(row).into(),
            )),
        ),
        8 => (
            change_event_id,
            data::ChangeEvent::Directory(data::DirectoryEvent::Delete(
                data::DirectoryDelete::from(row).into(),
            )),
        ),
        9 => (
            change_event_id,
            data::ChangeEvent::Directory(data::DirectoryEvent::UndoDelete(
                data::DirectoryUndoDelete::from(row).into(),
            )),
        ),
        10 => (
            change_event_id,
            data::ChangeEvent::Symlink(data::SymlinkEvent::Create(
                data::SymlinkCreate::from(row).into(),
            )),
        ),
        11 => (
            change_event_id,
            data::ChangeEvent::Symlink(data::SymlinkEvent::Delete(
                data::SymlinkDelete::from(row).into(),
            )),
        ),
        _ => panic!(),
    }
}

pub async fn get_changes(
    change_id_from: i32,
    change_id_to: i32,
    db_pool: &sqlx::PgPool,
) -> Result<Vec<(i32, data::ChangeEvent)>, sqlx::Error> {
    let mut changes_in_tables = vec![];

    for table in super::TABLES {
        let changes = sqlx::query(
            format!(
                r#"
            SELECT * FROM {} WHERE change_event_id > $1 AND change_event_id <= $2
        "#,
                table.table_name()
            )
            .as_str(),
        )
        .bind(change_id_from)
        .bind(change_id_to)
        .map(|row: sqlx::postgres::PgRow| row_to_change_event(row, &table))
        .fetch_all(db_pool)
        .await?;

        changes_in_tables.push(changes);
    }

    let changes = data::merge_changes(changes_in_tables);

    Ok(changes)
}

#[cfg(test)]
mod test {
    use super::get_changes;
    use crate::testing_utils::clear_tables_and_get_pool;

    #[tokio::test]
    async fn test_get_changes() {
        let db_pool = clear_tables_and_get_pool().await.unwrap();

        let changes = get_changes(0, 100, &db_pool).await.unwrap();

        assert_eq!(changes.len(), 0);
    }
}
