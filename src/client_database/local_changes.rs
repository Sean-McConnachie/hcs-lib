use crate::{client_database, data};
use std::{collections::LinkedList, fs};

pub fn read_changes(
    file_handler_config: &client_database::FileHandlerConfig,
) -> LinkedList<data::ChangeEvent> {
    let change_dir = file_handler_config.program_data_directory.join("changes");

    let mut all_changes = fs::read_dir(&change_dir)
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();
    all_changes.sort_by_key(|f| f.file_name());

    let mut changes = LinkedList::new();
    for change in all_changes {
        if change.path().metadata().unwrap().len() != 0 {
            changes.push_back(client_database::local_changes::parse_change(
                &fs::read_to_string(&change.path()).unwrap(),
            ));
        }
    }
    changes
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::{
        client_database,
        data::{self, InnerEventTrait},
        testing_utils,
    };

    use super::read_changes;

    #[test]
    fn test_parse_change() {
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
            data::ChangeEvent::Directory(data::DirectoryEvent::Create(data::DirectoryCreate::new(
                "hello".to_string()
            )))
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
            data::ChangeEvent::Directory(data::DirectoryEvent::Move(data::DirectoryMove::new(
                "hello".to_string(),
                "hello2".to_string()
            )))
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
            data::ChangeEvent::Directory(data::DirectoryEvent::Delete(data::DirectoryDelete::new(
                "hello".to_string()
            )))
        );
    }

    fn write_changes(changes: &[&str], change_counter: &mut client_database::ChangeCounter) {
        for change in changes {
            fs::write(&change_counter.next_path(), change).unwrap();
        }
    }

    #[test]
    fn test_get_changes() {
        let (file_handler_config, mut change_counter) =
            testing_utils::rm_dirs_ce_dirs_get_default_helpers();

        let changes = [
            "create_file\n1.txt",
            "create_file\n2.txt",
            "move_file\n1.txt\n1-1.txt",
        ];
        write_changes(&changes, &mut change_counter);

        let changes = read_changes(&file_handler_config);

        assert_eq!(changes.len(), 3);
    }

    #[test]
    fn test_get_chains() {
        let (file_handler_config, mut change_counter) =
            testing_utils::rm_dirs_ce_dirs_get_default_helpers();

        let changes = [
            "create_file\n1.txt",
            "create_file\n2.txt",
            "move_file\n1.txt\n1-1.txt",
            "delete_file\n3.txt",
            "create_file\n3.txt",
        ];
        write_changes(&changes, &mut change_counter);

        let changes = read_changes(&file_handler_config);
        let (chains, deleted_chains) = data::get_chains(changes);

        assert_eq!(chains.len(), 3);
        assert_eq!(chains.get("1-1.txt").unwrap().len(), 2);

        assert_eq!(deleted_chains.len(), 1);
    }

    #[test]
    fn test_optimize_create_to_delete() {
        let (file_handler_config, mut change_counter) =
            testing_utils::rm_dirs_ce_dirs_get_default_helpers();

        let changes = [
            "create_file\n1.txt",
            "move_file\n1.txt\n1-1.txt",
            "delete_file\n1-1.txt",
        ];
        write_changes(&changes, &mut change_counter);

        let changes = read_changes(&file_handler_config);
        let (_, mut deleted_chains) = data::get_chains(changes);

        assert_eq!(deleted_chains.front().unwrap().len(), 3);
        data::make_optimizations(deleted_chains.front_mut().unwrap());
        assert_eq!(deleted_chains.front().unwrap().len(), 0);
    }

    #[test]
    fn test_optimize_create_to_move() {
        let (file_handler_config, mut change_counter) =
            testing_utils::rm_dirs_ce_dirs_get_default_helpers();

        let changes = [
            "create_file\n1.txt",
            "move_file\n1.txt\n1-1.txt",
            "modify_file\n1-1.txt",
        ];
        write_changes(&changes, &mut change_counter);

        let changes = read_changes(&file_handler_config);
        let (mut chains, _) = data::get_chains(changes);

        assert_eq!(chains.get("1-1.txt").unwrap().len(), 3);
        data::make_optimizations(chains.get_mut("1-1.txt").unwrap());
        assert_eq!(chains.get("1-1.txt").unwrap().len(), 1);
        assert_eq!(
            chains
                .get("1-1.txt")
                .unwrap()
                .front()
                .unwrap()
                .inner_event(),
            data::InnerEvent::Create
        );
    }

    #[test]
    fn test_optimize_create_to_move_2() {
        let (file_handler_config, mut change_counter) =
            testing_utils::rm_dirs_ce_dirs_get_default_helpers();

        let changes = [
            "create_file\n1.txt",
            "modify_file\n1.txt",
            "move_file\n1.txt\n1-1.txt",
        ];
        write_changes(&changes, &mut change_counter);

        let changes = read_changes(&file_handler_config);
        let (mut chains, _) = data::get_chains(changes);

        assert_eq!(chains.get("1-1.txt").unwrap().len(), 3);
        data::make_optimizations(chains.get_mut("1-1.txt").unwrap());
        assert_eq!(chains.get("1-1.txt").unwrap().len(), 1);
        assert_eq!(
            chains
                .get("1-1.txt")
                .unwrap()
                .front()
                .unwrap()
                .inner_event(),
            data::InnerEvent::Create
        );
    }

    #[test]
    fn test_any_to_delete() {
        let (file_handler_config, mut change_counter) =
            testing_utils::rm_dirs_ce_dirs_get_default_helpers();

        let changes = [
            "modify_file\n1.txt",
            "move_file\n1.txt\n1-1.txt",
            "delete_file\n1-1.txt",
        ];
        write_changes(&changes, &mut change_counter);

        let changes = read_changes(&file_handler_config);
        let (_, mut deleted_chains) = data::get_chains(changes);

        assert_eq!(deleted_chains.front().unwrap().len(), 3);
        data::make_optimizations(deleted_chains.front_mut().unwrap());
        assert_eq!(deleted_chains.front().unwrap().len(), 2);
    }

    #[test]
    fn test_lots() {
        let (file_handler_config, mut change_counter) =
            testing_utils::rm_dirs_ce_dirs_get_default_helpers();

        let changes = [
            "create_file\na",
            "modify_file\na",
            "move_file\na\nb",
            "modify_file\nb",
            "modify_file\nb",
            "create_file\na",
            "modify_file\na",
            "move_file\na\ne",
        ];
        write_changes(&changes, &mut change_counter);

        let changes = read_changes(&file_handler_config);
        let (mut chains, _) = data::get_chains(changes);

        assert_eq!(chains.get("e").unwrap().len(), 3);
        data::make_optimizations(chains.get_mut("e").unwrap());
        assert_eq!(chains.get("e").unwrap().len(), 1);
        assert_eq!(
            chains.get("e").unwrap().front().unwrap().inner_event(),
            data::InnerEvent::Create
        );

        dbg!(&chains.get("b").unwrap());

        assert_eq!(chains.get("b").unwrap().len(), 5);
        data::make_optimizations(chains.get_mut("b").unwrap());
        assert_eq!(chains.get("b").unwrap().len(), 1);
    }
}

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
