use std::{
    collections::{HashMap, LinkedList},
    fs, path,
};

use crate::{
    client_database,
    data::{self, InnerEventTrait},
};

#[derive(Debug, Clone)]
pub struct ChangeFile {
    change: data::ChangeEvent,
    change_file: path::PathBuf,
}

impl ChangeFile {
    fn new(change: data::ChangeEvent, change_file: path::PathBuf) -> Self {
        Self {
            change,
            change_file,
        }
    }

    fn change(&self) -> &data::ChangeEvent {
        &self.change
    }

    fn change_file(&self) -> &path::PathBuf {
        &self.change_file
    }
}

/// Returns:
///     [0] = Entry if exists in HashMap
///    [1] = New path to use in HashMap
///   [2] = Whether this entry is a delete
fn get_chain_path(change: &data::ChangeEvent) -> (String, Option<String>, bool) {
    match change {
        data::ChangeEvent::File(file) => match file {
            data::FileEvent::Create(file_create) => (file_create.path().to_string(), None, false),
            data::FileEvent::Modify(file_modify) => (file_modify.path().to_string(), None, false),
            data::FileEvent::Move(file_move) => (
                file_move.from_path().to_string(),
                Some(file_move.to_path().to_string()),
                false,
            ),
            data::FileEvent::Delete(file_delete) => (file_delete.path().to_string(), None, true),
            data::FileEvent::UndoDelete(_) => unimplemented!(),
        },
        data::ChangeEvent::Directory(dir) => match dir {
            data::DirectoryEvent::Create(dir_create) => {
                (dir_create.path().to_string(), None, false)
            }
            data::DirectoryEvent::Move(dir_move) => (
                dir_move.from_path().to_string(),
                Some(dir_move.to_path().to_string()),
                false,
            ),
            data::DirectoryEvent::Delete(dir_delete) => (dir_delete.path().to_string(), None, true),
            data::DirectoryEvent::UndoDelete(_) => unimplemented!(),
        },
        data::ChangeEvent::Symlink(_) => unimplemented!("symlinks not implemented"),
    }
}

fn get_changes(file_handler_config: &client_database::FileHandlerConfig) -> LinkedList<ChangeFile> {
    let change_dir = file_handler_config.program_data_directory.join("changes");

    let mut all_changes = fs::read_dir(&change_dir)
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();
    all_changes.sort_by_key(|f| f.file_name());

    let mut changes = LinkedList::new();
    for change in all_changes {
        if change.path().metadata().unwrap().len() != 0 {
            changes.push_back(ChangeFile::new(
                client_database::local_changes::parse_change(
                    &fs::read_to_string(&change.path()).unwrap(),
                ),
                change.path().to_path_buf(),
            ));
        }
    }
    changes
}

fn get_chains(
    mut changes: LinkedList<ChangeFile>,
) -> (
    HashMap<String, LinkedList<ChangeFile>>,
    LinkedList<LinkedList<ChangeFile>>,
) {
    let mut deleted_chains = LinkedList::new();
    let mut chains: HashMap<String, LinkedList<ChangeFile>> = HashMap::new();

    loop {
        let change = match changes.pop_front() {
            Some(change) => change,
            None => break,
        };
        let (existing_path, new_path, is_delete) = get_chain_path(change.change());
        if let Some(chain) = chains.get_mut(&existing_path) {
            chain.push_back(change);
        } else {
            let mut current_chain = LinkedList::new();
            current_chain.push_back(change);
            chains.insert(existing_path.clone(), current_chain);
        }

        if is_delete {
            let current_chain = chains.remove(&existing_path).unwrap();
            deleted_chains.push_back(current_chain);
        } else if let Some(new_path) = new_path {
            let chain = chains.remove(&existing_path).unwrap();
            chains.insert(new_path, chain);
        }
    }
    (chains, deleted_chains)
}

fn get_most_recent_move_change(chain: &mut LinkedList<ChangeFile>) -> Option<ChangeFile> {
    for change in chain.iter().rev() {
        if change.change().inner_event() == data::InnerEvent::Move {
            return Some(change.clone());
        }
    }
    None
}

fn get_first_modify_change(chain: &mut LinkedList<ChangeFile>) -> Option<ChangeFile> {
    for change in chain.iter() {
        if change.change().inner_event() == data::InnerEvent::Modify {
            return Some(change.clone());
        }
    }
    None
}

fn move_to_from_of_change(change: &ChangeFile) -> (String, String) {
    match change.change() {
        data::ChangeEvent::File(data::FileEvent::Move(file_move)) => (
            file_move.from_path().to_string(),
            file_move.to_path().to_string(),
        ),
        data::ChangeEvent::Directory(data::DirectoryEvent::Move(dir_move)) => (
            dir_move.from_path().to_string(),
            dir_move.to_path().to_string(),
        ),
        _ => panic!("change is not a move"),
    }
}

fn path_of_modify_change(change: &ChangeFile) -> String {
    match change.change() {
        data::ChangeEvent::File(data::FileEvent::Modify(file_modify)) => {
            file_modify.path().to_string()
        }
        _ => panic!("change is not a modify"),
    }
}

/// Create: C
/// Modify: Y
/// Move: M
/// Delete: D
///
/// [0] C -> Y = C
/// [1] C -> M = C @ M
/// [2] C -> D = null
///
/// [3] Y -> M = M -> Y
/// [4] Y -> D = D
/// [5] Y_1 -> Y_k = Y_k
///
/// [6] M_1 -> M_k = M_1_from -> M_k_to
/// [7] M_1 -> D = D @ M_1_from
/// [8] M -> Y = M -> Y
fn make_optimizations(chain: &mut LinkedList<ChangeFile>) {
    if chain.len() == 1 {
        return;
    }

    let is_dir = match chain.front().unwrap().change() {
        data::ChangeEvent::File(_) => false,
        data::ChangeEvent::Directory(_) => true,
        data::ChangeEvent::Symlink(_) => unimplemented!("symlinks not implemented"),
    };

    if chain.front().unwrap().change().inner_event() == data::InnerEvent::Create {
        if chain.back().unwrap().change().inner_event() == data::InnerEvent::Delete {
            // [2] C -> D = null
            chain.clear();
            return;
        }

        if let Some(move_change) = get_most_recent_move_change(chain) {
            let (_, move_to) = move_to_from_of_change(&move_change);

            let optimized_create = match is_dir {
                false => data::ChangeEvent::File(data::FileEvent::Create(data::FileCreate::new(
                    0, move_to,
                ))),
                true => data::ChangeEvent::Directory(data::DirectoryEvent::Create(
                    data::DirectoryCreate::new(move_to),
                )),
            };
            chain.clear();
            chain.push_back(ChangeFile::new(
                optimized_create,
                move_change.change_file().clone(),
            ));
            return;
        }
    } else if chain.back().unwrap().change().inner_event() == data::InnerEvent::Delete {
        let original_change_file_path = chain.front().unwrap().change_file().clone();
        let from_path = match chain.front().unwrap().change() {
            data::ChangeEvent::File(file_event) => match file_event {
                data::FileEvent::Modify(file_modify) => file_modify.path(),
                data::FileEvent::Move(file_move) => file_move.from_path(),
                _ => unreachable!("change is not a modify or move"),
            },
            data::ChangeEvent::Directory(dir_event) => match dir_event {
                data::DirectoryEvent::Move(dir_move) => dir_move.from_path(),
                _ => unreachable!("change is not a modify or move"),
            },
            data::ChangeEvent::Symlink(_) => unreachable!("symlink not implemented"),
        };

        let delete_path = match chain.back().unwrap().change() {
            data::ChangeEvent::File(data::FileEvent::Delete(file_delete)) => file_delete.path(),
            data::ChangeEvent::Directory(data::DirectoryEvent::Delete(dir_delete)) => {
                dir_delete.path()
            }
            data::ChangeEvent::Symlink(_) => unreachable!("symlink not implemented"),
            _ => unreachable!("change is not a delete"),
        };

        if from_path == delete_path {
            let delete = chain.pop_back().unwrap();
            chain.clear();
            chain.push_back(delete);
            return;
        } else {
            let move_change = match is_dir {
                false => data::ChangeEvent::File(data::FileEvent::Move(data::FileMove::new(
                    from_path.to_string(),
                    delete_path.to_string(),
                ))),
                true => data::ChangeEvent::Directory(data::DirectoryEvent::Move(
                    data::DirectoryMove::new(from_path.to_string(), delete_path.to_string()),
                )),
            };
            let delete_change = match is_dir {
                false => data::ChangeEvent::File(data::FileEvent::Delete(data::FileDelete::new(
                    delete_path.to_string(),
                ))),
                true => data::ChangeEvent::Directory(data::DirectoryEvent::Delete(
                    data::DirectoryDelete::new(delete_path.to_string()),
                )),
            };

            chain.clear();
            chain.push_back(ChangeFile::new(
                move_change,
                original_change_file_path.clone(),
            ));
            chain.push_back(ChangeFile::new(delete_change, original_change_file_path));
            return;
        }
    } else {
        // this chain could contain a modify and/or move which are both relevant to the optimization
        let move_change = get_most_recent_move_change(chain);
        let modify_change = get_first_modify_change(chain);

        if move_change.is_none() && modify_change.is_none() {
            return;
        } else if !move_change.is_none() && !modify_change.is_none() {
            let original_change_path = chain.front().unwrap().change_file().clone();
            let modify_change = modify_change.unwrap();
            let move_change = move_change.unwrap();

            let move_from = path_of_modify_change(&modify_change);
            let (_, move_to) = move_to_from_of_change(&move_change);

            let optimized_modify = match is_dir {
                false => data::ChangeEvent::File(data::FileEvent::Modify(data::FileModify::new(
                    0,
                    move_to.clone(),
                ))),
                true => unreachable!(),
            };

            let optimized_move = match is_dir {
                false => data::ChangeEvent::File(data::FileEvent::Move(data::FileMove::new(
                    move_from.to_string(),
                    move_to.clone(),
                ))),
                true => unreachable!(),
            };

            chain.clear();
            chain.push_back(ChangeFile::new(
                optimized_move,
                original_change_path.clone(),
            ));
            chain.push_back(ChangeFile::new(
                optimized_modify,
                original_change_path.clone(),
            ));
            return;
        } else if move_change.is_none() && !modify_change.is_none() {
            let modify_change = modify_change.unwrap();
            chain.clear();
            chain.push_back(modify_change);
        } else if !move_change.is_none() && modify_change.is_none() {
            let original_change_file_path = chain.front().unwrap().change_file().clone();
            let move_change = move_change.unwrap();

            let move_from = match chain.front().unwrap().change() {
                data::ChangeEvent::File(file_event) => match file_event {
                    data::FileEvent::Move(file_move) => file_move.from_path(),
                    _ => unreachable!("change is not a modify or move"),
                },
                data::ChangeEvent::Directory(dir_event) => match dir_event {
                    data::DirectoryEvent::Move(dir_move) => dir_move.from_path(),
                    _ => unreachable!("change is not a modify or move"),
                },
                _ => unreachable!("change is not a modify"),
            };

            let (_, move_to) = move_to_from_of_change(&move_change);

            let optimized_move = match is_dir {
                false => data::ChangeEvent::File(data::FileEvent::Move(data::FileMove::new(
                    move_from.to_string(),
                    move_to.clone(),
                ))),
                true => unreachable!(),
            };

            chain.clear();
            chain.push_back(ChangeFile::new(
                optimized_move,
                original_change_file_path.clone(),
            ));
            return;
        }
    }
}

pub fn optimize_changes(
    file_handler_config: &client_database::FileHandlerConfig,
) -> Vec<LinkedList<ChangeFile>> {
    let changes = get_changes(file_handler_config);

    let (chains, deleted_chains) = get_chains(changes);

    let mut optimized_changes = vec![];

    for (_last_path, mut chain) in chains.into_iter() {
        make_optimizations(&mut chain);
        optimized_changes.push(chain);
    }

    for mut chain in deleted_chains.into_iter() {
        make_optimizations(&mut chain);
        optimized_changes.push(chain)
    }

    optimized_changes
}

#[cfg(test)]
mod test {
    use crate::testing_utils;

    use super::*;

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

        let changes = get_changes(&file_handler_config);

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

        let changes = get_changes(&file_handler_config);
        let (chains, deleted_chains) = get_chains(changes);

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

        let changes = get_changes(&file_handler_config);
        let (_, mut deleted_chains) = get_chains(changes);

        assert_eq!(deleted_chains.front().unwrap().len(), 3);
        make_optimizations(deleted_chains.front_mut().unwrap());
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

        let changes = get_changes(&file_handler_config);
        let (mut chains, _) = get_chains(changes);

        assert_eq!(chains.get("1-1.txt").unwrap().len(), 3);
        make_optimizations(chains.get_mut("1-1.txt").unwrap());
        assert_eq!(chains.get("1-1.txt").unwrap().len(), 1);
        assert_eq!(
            chains
                .get("1-1.txt")
                .unwrap()
                .front()
                .unwrap()
                .change()
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

        let changes = get_changes(&file_handler_config);
        let (mut chains, _) = get_chains(changes);

        assert_eq!(chains.get("1-1.txt").unwrap().len(), 3);
        make_optimizations(chains.get_mut("1-1.txt").unwrap());
        assert_eq!(chains.get("1-1.txt").unwrap().len(), 1);
        assert_eq!(
            chains
                .get("1-1.txt")
                .unwrap()
                .front()
                .unwrap()
                .change()
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

        let changes = get_changes(&file_handler_config);
        let (_, mut deleted_chains) = get_chains(changes);

        assert_eq!(deleted_chains.front().unwrap().len(), 3);
        make_optimizations(deleted_chains.front_mut().unwrap());
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

        let changes = get_changes(&file_handler_config);
        let (mut chains, _) = get_chains(changes);

        assert_eq!(chains.get("e").unwrap().len(), 3);
        make_optimizations(chains.get_mut("e").unwrap());
        assert_eq!(chains.get("e").unwrap().len(), 1);
        assert_eq!(
            chains
                .get("e")
                .unwrap()
                .front()
                .unwrap()
                .change()
                .inner_event(),
            data::InnerEvent::Create
        );

        dbg!(&chains.get("b").unwrap());

        assert_eq!(chains.get("b").unwrap().len(), 5);
        make_optimizations(chains.get_mut("b").unwrap());
        assert_eq!(chains.get("b").unwrap().len(), 1);
    }
}
