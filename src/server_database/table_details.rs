use crate::data;

pub const TABLES: [TableDetails; 11] = [
    TableDetails {
        table_description: "File Create",
        change_type_id: 1,
        table_name: "file_create",
    },
    TableDetails {
        table_description: "File Modify",
        change_type_id: 2,
        table_name: "file_modify",
    },
    TableDetails {
        table_description: "File Move",
        change_type_id: 3,
        table_name: "file_move",
    },
    TableDetails {
        table_description: "File Delete",
        change_type_id: 4,
        table_name: "file_delete",
    },
    TableDetails {
        table_description: "Undo File Delete",
        change_type_id: 5,
        table_name: "undo_file_delete",
    },
    TableDetails {
        table_description: "Directory Create",
        change_type_id: 6,
        table_name: "directory_create",
    },
    TableDetails {
        table_description: "Directory Move",
        change_type_id: 7,
        table_name: "directory_move",
    },
    TableDetails {
        table_description: "Directory Delete",
        change_type_id: 8,
        table_name: "directory_delete",
    },
    TableDetails {
        table_description: "Undo Directory Delete",
        change_type_id: 9,
        table_name: "undo_directory_delete",
    },
    TableDetails {
        table_description: "Symlink Create",
        change_type_id: 10,
        table_name: "symlink_create",
    },
    TableDetails {
        table_description: "Symlink Delete",
        change_type_id: 11,
        table_name: "symlink_delete",
    },
];

#[derive(Debug, PartialEq, Clone)]
pub struct TableDetails {
    table_description: &'static str,
    change_type_id: i32,
    table_name: &'static str,
}

impl Copy for TableDetails {}

impl TableDetails {
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    pub fn change_type_id(&self) -> i32 {
        self.change_type_id
    }

    pub fn table_description(&self) -> &str {
        &self.table_description
    }
}

pub trait TableDetailsTrait {
    fn table_details(&self) -> &TableDetails;
}

impl TableDetailsTrait for data::ChangeEvent {
    fn table_details(&self) -> &TableDetails {
        match self {
            data::ChangeEvent::File(file_event) => match file_event {
                data::FileEvent::Create(_) => &TABLES[0],
                data::FileEvent::Modify(_) => &TABLES[1],
                data::FileEvent::Move(_) => &TABLES[2],
                data::FileEvent::Delete(_) => &TABLES[3],
                data::FileEvent::UndoDelete(_) => &TABLES[4],
            },
            data::ChangeEvent::Directory(dir_event) => match dir_event {
                data::DirectoryEvent::Create(_) => &TABLES[5],
                data::DirectoryEvent::Move(_) => &TABLES[6],
                data::DirectoryEvent::Delete(_) => &TABLES[7],
                data::DirectoryEvent::UndoDelete(_) => &TABLES[8],
            },
            data::ChangeEvent::Symlink(symlink_event) => match symlink_event {
                data::SymlinkEvent::Create(_) => &TABLES[9],
                data::SymlinkEvent::Delete(_) => &TABLES[10],
            },
        }
    }
}
