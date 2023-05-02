use crate::data;

pub fn parse_change(text: &str) -> data::ChangeEvent {
    let first_line = text.lines().next().unwrap();
    match first_line {
        "create_file" => {
            let created_file = text.lines().nth(1).unwrap().to_string();
            data::ChangeEvent::File(data::FileEvent::Create(data::FileCreate::new(
                0,
                created_file,
            )))
        }
        "create_dir" => {
            let created_dir = text.lines().nth(1).unwrap().to_string();
            data::ChangeEvent::Directory(data::DirectoryEvent::Create(data::DirectoryCreate::new(
                created_dir,
            )))
        }
        "modify_file" => {
            let modified_file = text.lines().nth(1).unwrap().to_string();
            data::ChangeEvent::File(data::FileEvent::Modify(data::FileModify::new(
                0,
                modified_file,
            )))
        }
        "move_file" => {
            let from_path = text.lines().nth(1).unwrap().to_string();
            let to_path = text.lines().nth(2).unwrap().to_string();
            data::ChangeEvent::File(data::FileEvent::Move(data::FileMove::new(
                from_path, to_path,
            )))
        }
        "move_dir" => {
            let from_path = text.lines().nth(1).unwrap().to_string();
            let to_path = text.lines().nth(2).unwrap().to_string();
            data::ChangeEvent::Directory(data::DirectoryEvent::Move(data::DirectoryMove::new(
                from_path, to_path,
            )))
        }
        "delete_file" => {
            let deleted_file = text.lines().nth(1).unwrap().to_string();
            data::ChangeEvent::File(data::FileEvent::Delete(data::FileDelete::new(deleted_file)))
        }
        "delete_dir" => {
            let deleted_dir = text.lines().nth(1).unwrap().to_string();
            data::ChangeEvent::Directory(data::DirectoryEvent::Delete(data::DirectoryDelete::new(
                deleted_dir,
            )))
        }
        _ => unimplemented!("Unimplemented change type: `{}`", first_line,),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_change() {
        let change = "create_file\nhello.txt";
        let change = super::parse_change(change);
        assert_eq!(
            change,
            data::ChangeEvent::File(data::FileEvent::Create(data::FileCreate::new(
                0,
                "hello.txt".to_string()
            )))
        );

        let change = "create_dir\nhello";
        let change = super::parse_change(change);
        assert_eq!(
            change,
            data::ChangeEvent::Directory(data::DirectoryEvent::Create(
                data::DirectoryCreate::new("hello".to_string())
            ))
        );

        let change = "modify_file\nhello.txt";
        let change = super::parse_change(change);
        assert_eq!(
            change,
            data::ChangeEvent::File(data::FileEvent::Modify(data::FileModify::new(
                0,
                "hello.txt".to_string()
            )))
        );

        let change = "move_file\nhello.txt\nhello2.txt";
        let change = super::parse_change(change);
        assert_eq!(
            change,
            data::ChangeEvent::File(data::FileEvent::Move(data::FileMove::new(
                "hello.txt".to_string(),
                "hello2.txt".to_string()
            )))
        );

        let change = "move_dir\nhello\nhello2";
        let change = super::parse_change(change);
        assert_eq!(
            change,
            data::ChangeEvent::Directory(data::DirectoryEvent::Move(
                data::DirectoryMove::new("hello".to_string(), "hello2".to_string())
            ))
        );

        let change = "delete_file\nhello.txt";
        let change = super::parse_change(change);
        assert_eq!(
            change,
            data::ChangeEvent::File(data::FileEvent::Delete(data::FileDelete::new(
                "hello.txt".to_string()
            )))
        );

        let change = "delete_dir\nhello";
        let change = super::parse_change(change);
        assert_eq!(
            change,
            data::ChangeEvent::Directory(data::DirectoryEvent::Delete(
                data::DirectoryDelete::new("hello".to_string())
            ))
        );
    }
}

// pub trait LocalChangeFile {
//     fn file_name(&self) -> &str;
// }

// pub enum LocalChange {
//     CreateFile(CreateFile),
//     CreateDir(CreateDir),
//     ModifyFile(ModifyFile),
//     ModifyDir(ModifyDir),
//     MoveFile(MoveFile),
//     MoveDir(MoveDir),
//     DeleteFile(DeleteFile),
//     DeleteDir(DeleteDir),
// }

// impl LocalChange {
//     pub fn file_name(&self) -> &str {
//         match self {
//             LocalChange::CreateFile(change) => change.file_name(),
//             LocalChange::CreateDir(change) => change.dir_name(),
//             LocalChange::ModifyFile(change) => change.file_name(),
//             LocalChange::ModifyDir(change) => change.dir_name(),
//             LocalChange::MoveFile(change) => change.old_file_name(),
//             LocalChange::MoveDir(change) => change.old_dir_name(),
//             LocalChange::DeleteFile(change) => change.file_name(),
//             LocalChange::DeleteDir(change) => change.dir_name(),
//         }
//     }
// }

// pub struct CreateFile {
//     file_name: String,
//     local_change_file: String,
// }

// impl CreateFile {
//     pub fn new(file_name: String) -> Self {
//         Self { file_name }
//     }

//     pub fn file_name(&self) -> &str {
//         &self.file_name
//     }
// }

// impl LocalChangeFile for CreateFile {
//     fn file_name(&self) -> &str {
//         &self.local_change_file
//     }
// }

// pub struct CreateDir {
//     dir_name: String,
//     local_change_file: String,
// }

// impl CreateDir {
//     pub fn new(dir_name: String) -> Self {
//         Self { dir_name }
//     }

//     pub fn dir_name(&self) -> &str {
//         &self.dir_name
//     }
// }

// impl LocalChangeFile for CreateDir {
//     fn file_name(&self) -> &str {
//         &self.local_change_file
//     }
// }

// pub struct ModifyFile {
//     file_name: String,
//     local_change_file: String,
// }

// impl ModifyFile {
//     pub fn new(file_name: String) -> Self {
//         Self { file_name }
//     }

//     pub fn file_name(&self) -> &str {
//         &self.file_name
//     }
// }

// impl LocalChangeFile for ModifyFile {
//     fn file_name(&self) -> &str {
//         &self.local_change_file
//     }
// }

// pub struct ModifyDir {
//     dir_name: String,
//     local_change_file: String,
// }

// impl ModifyDir {
//     pub fn new(dir_name: String) -> Self {
//         Self { dir_name }
//     }

//     pub fn dir_name(&self) -> &str {
//         &self.dir_name
//     }
// }

// impl LocalChangeFile for ModifyDir {
//     fn file_name(&self) -> &str {
//         &self.local_change_file
//     }
// }

// pub struct MoveFile {
//     old_file_name: String,
//     new_file_name: String,
//     local_change_file: String,
// }

// impl MoveFile {
//     pub fn new(old_file_name: String, new_file_name: String) -> Self {
//         Self {
//             old_file_name,
//             new_file_name,
//         }
//     }

//     pub fn old_file_name(&self) -> &str {
//         &self.old_file_name
//     }

//     pub fn new_file_name(&self) -> &str {
//         &self.new_file_name
//     }
// }

// impl LocalChangeFile for MoveFile {
//     fn file_name(&self) -> &str {
//         &self.local_change_file
//     }
// }

// pub struct MoveDir {
//     old_dir_name: String,
//     new_dir_name: String,
//     local_change_file: String,
// }

// impl MoveDir {
//     pub fn new(old_dir_name: String, new_dir_name: String) -> Self {
//         Self {
//             old_dir_name,
//             new_dir_name,
//         }
//     }

//     pub fn old_dir_name(&self) -> &str {
//         &self.old_dir_name
//     }

//     pub fn new_dir_name(&self) -> &str {
//         &self.new_dir_name
//     }
// }

// impl LocalChangeFile for MoveDir {
//     fn file_name(&self) -> &str {
//         &self.local_change_file
//     }
// }

// pub struct DeleteFile {
//     file_name: String,
//     local_change_file: String,
// }

// impl DeleteFile {
//     pub fn new(file_name: String) -> Self {
//         Self { file_name }
//     }

//     pub fn file_name(&self) -> &str {
//         &self.file_name
//     }
// }

// impl LocalChangeFile for DeleteFile {
//     fn file_name(&self) -> &str {
//         &self.local_change_file
//     }
// }

// pub struct DeleteDir {
//     dir_name: String,
//     local_change_file: String,
// }

// impl DeleteDir {
//     pub fn new(dir_name: String) -> Self {
//         Self { dir_name }
//     }

//     pub fn dir_name(&self) -> &str {
//         &self.dir_name
//     }
// }

// impl LocalChangeFile for DeleteDir {
//     fn file_name(&self) -> &str {
//         &self.local_change_file
//     }
// }
