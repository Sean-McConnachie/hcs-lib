use crate::data::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ChangeType {
    FileCreate(FileCreate),
    FileModify(FileModify),
    FileMove(FileMove),
    FileDelete(FileDelete),
    FileUndoDelete(FileUndoDelete),

    SymlinkCreate(SymlinkCreate),
    SymlinkDelete(SymlinkDelete),

    DirectoryCreate(DirectoryCreate),
    DirectoryMove(DirectoryMove),
    DirectoryDelete(DirectoryDelete),
    DirectoryUndoDelete(DirectoryUndoDelete),
}

impl ChangeType {
    #[allow(dead_code)]
    fn data_uid(&self) -> u16 {
        match self {
            ChangeType::FileCreate(_) => 5,
            ChangeType::FileModify(_) => 6,
            ChangeType::FileMove(_) => 7,
            ChangeType::FileDelete(_) => 8,
            ChangeType::FileUndoDelete(_) => 9,
            ChangeType::SymlinkCreate(_) => 10,
            ChangeType::SymlinkDelete(_) => 11,
            ChangeType::DirectoryCreate(_) => 12,
            ChangeType::DirectoryMove(_) => 13,
            ChangeType::DirectoryDelete(_) => 14,
            ChangeType::DirectoryUndoDelete(_) => 15,
        }
    }
}

// Copilot is a bloody legend :)
impl From<ChangeType> for FileCreate {
    fn from(change: ChangeType) -> Self {
        match change {
            ChangeType::FileCreate(file_create) => file_create,
            _ => panic!("ChangeType is not FileCreate"),
        }
    }
}

impl From<ChangeType> for FileModify {
    fn from(change: ChangeType) -> Self {
        match change {
            ChangeType::FileModify(file_modify) => file_modify,
            _ => panic!("ChangeType is not FileModify"),
        }
    }
}

impl From<ChangeType> for FileMove {
    fn from(change: ChangeType) -> Self {
        match change {
            ChangeType::FileMove(file_move) => file_move,
            _ => panic!("ChangeType is not FileMove"),
        }
    }
}

impl From<ChangeType> for FileDelete {
    fn from(change: ChangeType) -> Self {
        match change {
            ChangeType::FileDelete(file_delete) => file_delete,
            _ => panic!("ChangeType is not FileDelete"),
        }
    }
}

impl From<ChangeType> for FileUndoDelete {
    fn from(change: ChangeType) -> Self {
        match change {
            ChangeType::FileUndoDelete(file_undo_delete) => file_undo_delete,
            _ => panic!("ChangeType is not FileUndoDelete"),
        }
    }
}

impl From<ChangeType> for SymlinkCreate {
    fn from(change: ChangeType) -> Self {
        match change {
            ChangeType::SymlinkCreate(symlink_create) => symlink_create,
            _ => panic!("ChangeType is not SymlinkCreate"),
        }
    }
}

impl From<ChangeType> for SymlinkDelete {
    fn from(change: ChangeType) -> Self {
        match change {
            ChangeType::SymlinkDelete(symlink_delete) => symlink_delete,
            _ => panic!("ChangeType is not SymlinkDelete"),
        }
    }
}

impl From<ChangeType> for DirectoryCreate {
    fn from(change: ChangeType) -> Self {
        match change {
            ChangeType::DirectoryCreate(directory_create) => directory_create,
            _ => panic!("ChangeType is not DirectoryCreate"),
        }
    }
}

impl From<ChangeType> for DirectoryMove {
    fn from(change: ChangeType) -> Self {
        match change {
            ChangeType::DirectoryMove(directory_move) => directory_move,
            _ => panic!("ChangeType is not DirectoryMove"),
        }
    }
}

impl From<ChangeType> for DirectoryDelete {
    fn from(change: ChangeType) -> Self {
        match change {
            ChangeType::DirectoryDelete(directory_delete) => directory_delete,
            _ => panic!("ChangeType is not DirectoryDelete"),
        }
    }
}

impl From<ChangeType> for DirectoryUndoDelete {
    fn from(change: ChangeType) -> Self {
        match change {
            ChangeType::DirectoryUndoDelete(directory_undo_delete) => directory_undo_delete,
            _ => panic!("ChangeType is not DirectoryUndoDelete"),
        }
    }
}

impl Into<ChangeType> for FileCreate {
    fn into(self) -> ChangeType {
        ChangeType::FileCreate(self)
    }
}

impl Into<ChangeType> for FileModify {
    fn into(self) -> ChangeType {
        ChangeType::FileModify(self)
    }
}

impl Into<ChangeType> for FileMove {
    fn into(self) -> ChangeType {
        ChangeType::FileMove(self)
    }
}

impl Into<ChangeType> for FileDelete {
    fn into(self) -> ChangeType {
        ChangeType::FileDelete(self)
    }
}

impl Into<ChangeType> for FileUndoDelete {
    fn into(self) -> ChangeType {
        ChangeType::FileUndoDelete(self)
    }
}

impl Into<ChangeType> for SymlinkCreate {
    fn into(self) -> ChangeType {
        ChangeType::SymlinkCreate(self)
    }
}

impl Into<ChangeType> for SymlinkDelete {
    fn into(self) -> ChangeType {
        ChangeType::SymlinkDelete(self)
    }
}

impl Into<ChangeType> for DirectoryCreate {
    fn into(self) -> ChangeType {
        ChangeType::DirectoryCreate(self)
    }
}

impl Into<ChangeType> for DirectoryMove {
    fn into(self) -> ChangeType {
        ChangeType::DirectoryMove(self)
    }
}

impl Into<ChangeType> for DirectoryDelete {
    fn into(self) -> ChangeType {
        ChangeType::DirectoryDelete(self)
    }
}

impl Into<ChangeType> for DirectoryUndoDelete {
    fn into(self) -> ChangeType {
        ChangeType::DirectoryUndoDelete(self)
    }
}
