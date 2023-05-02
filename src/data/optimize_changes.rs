use std::collections::{HashMap, LinkedList};

use crate::data::{self, InnerEventTrait};

use super::ChangeEvent;

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

pub fn get_chains(
    mut changes: LinkedList<ChangeEvent>,
) -> (
    HashMap<String, LinkedList<ChangeEvent>>,
    LinkedList<LinkedList<ChangeEvent>>,
) {
    let mut deleted_chains = LinkedList::new();
    let mut chains: HashMap<String, LinkedList<ChangeEvent>> = HashMap::new();

    loop {
        let change = match changes.pop_front() {
            Some(change) => change,
            None => break,
        };
        let (existing_path, new_path, is_delete) = get_chain_path(&change);
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

fn get_most_recent_move_change(chain: &mut LinkedList<ChangeEvent>) -> Option<ChangeEvent> {
    for change in chain.iter().rev() {
        if change.inner_event() == data::InnerEvent::Move {
            return Some(change.clone());
        }
    }
    None
}

fn get_first_modify_change(chain: &mut LinkedList<ChangeEvent>) -> Option<ChangeEvent> {
    for change in chain.iter() {
        if change.inner_event() == data::InnerEvent::Modify {
            return Some(change.clone());
        }
    }
    None
}

fn move_to_from_of_change(change: &ChangeEvent) -> (String, String) {
    match change {
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

fn path_of_modify_change(change: &ChangeEvent) -> String {
    match change {
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
pub fn make_optimizations(chain: &mut LinkedList<ChangeEvent>) {
    if chain.len() == 1 {
        return;
    }

    let is_dir = match chain.front().unwrap() {
        data::ChangeEvent::File(_) => false,
        data::ChangeEvent::Directory(_) => true,
        data::ChangeEvent::Symlink(_) => unimplemented!("symlinks not implemented"),
    };

    if chain.front().unwrap().inner_event() == data::InnerEvent::Create {
        if chain.back().unwrap().inner_event() == data::InnerEvent::Delete {
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
            chain.push_back(optimized_create);
            return;
        }
    } else if chain.back().unwrap().inner_event() == data::InnerEvent::Delete {
        let from_path = match chain.front().unwrap() {
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

        let delete_path = match chain.back().unwrap() {
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
            chain.push_back(move_change);
            chain.push_back(delete_change);
            return;
        }
    } else {
        // this chain could contain a modify and/or move which are both relevant to the optimization
        let move_change = get_most_recent_move_change(chain);
        let modify_change = get_first_modify_change(chain);

        if move_change.is_none() && modify_change.is_none() {
            return;
        } else if !move_change.is_none() && !modify_change.is_none() {
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
            chain.push_back(optimized_move);
            chain.push_back(optimized_modify);
            return;
        } else if move_change.is_none() && !modify_change.is_none() {
            let modify_change = modify_change.unwrap();
            chain.clear();
            chain.push_back(modify_change);
        } else if !move_change.is_none() && modify_change.is_none() {
            let move_change = move_change.unwrap();

            let move_from = match chain.front().unwrap() {
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
            chain.push_back(optimized_move);
            return;
        }
    }
}

pub fn optimize_changes(changes: LinkedList<ChangeEvent>) -> Vec<LinkedList<ChangeEvent>> {
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
