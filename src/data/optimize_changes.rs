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
    mut changes: LinkedList<(i32, ChangeEvent)>,
) -> (
    HashMap<String, LinkedList<(i32, ChangeEvent)>>,
    LinkedList<LinkedList<(i32, ChangeEvent)>>,
) {
    let mut deleted_chains = LinkedList::new();
    let mut chains: HashMap<String, LinkedList<(i32, ChangeEvent)>> = HashMap::new();

    loop {
        let change = match changes.pop_front() {
            Some(change) => change,
            None => break,
        };
        let (existing_path, new_path, is_delete) = get_chain_path(&change.1);
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

fn get_most_recent_move_change(
    chain: &mut LinkedList<(i32, ChangeEvent)>,
) -> Option<(i32, ChangeEvent)> {
    for change in chain.iter().rev() {
        if change.1.inner_event() == data::InnerEvent::Move {
            return Some(change.clone());
        }
    }
    None
}

fn get_first_modify_change(
    chain: &mut LinkedList<(i32, ChangeEvent)>,
) -> Option<(i32, ChangeEvent)> {
    for change in chain.iter() {
        if change.1.inner_event() == data::InnerEvent::Modify {
            return Some(change.clone());
        }
    }
    None
}

fn move_to_from_of_change(change: &(i32, ChangeEvent)) -> (String, String) {
    match &change.1 {
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

fn path_of_modify_change(change: &(i32, ChangeEvent)) -> String {
    match &change.1 {
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
pub fn make_optimizations(chain: &mut LinkedList<(i32, ChangeEvent)>) -> Vec<(i32, ChangeEvent)> {
    if chain.len() == 1 {
        return vec![chain.front().unwrap().clone()];
    }

    let is_dir = match chain.front().unwrap().1 {
        data::ChangeEvent::File(_) => false,
        data::ChangeEvent::Directory(_) => true,
        data::ChangeEvent::Symlink(_) => unimplemented!("symlinks not implemented"),
    };

    if chain.front().unwrap().1.inner_event() == data::InnerEvent::Create {
        if chain.back().unwrap().1.inner_event() == data::InnerEvent::Delete {
            // [2] C -> D = null
            return vec![];
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
            return vec![(move_change.0, optimized_create)];
        }
    } else if chain.back().unwrap().1.inner_event() == data::InnerEvent::Delete {
        let delete_change_id = chain.back().unwrap().0;
        let start_change_id = chain.front().unwrap().0;
        let from_path = match &chain.front().unwrap().1 {
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

        let delete_path = match &chain.back().unwrap().1 {
            data::ChangeEvent::File(data::FileEvent::Delete(file_delete)) => file_delete.path(),
            data::ChangeEvent::Directory(data::DirectoryEvent::Delete(dir_delete)) => {
                dir_delete.path()
            }
            data::ChangeEvent::Symlink(_) => unreachable!("symlink not implemented"),
            _ => unreachable!("change is not a delete"),
        };

        if from_path == delete_path {
            let delete = chain.pop_back().unwrap();
            return vec![delete];
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

            return vec![
                (start_change_id, move_change),
                (delete_change_id, delete_change),
            ];
        }
    } else {
        // this chain could contain a modify and/or move which are both relevant to the optimization
        let move_change = get_most_recent_move_change(chain);
        let modify_change = get_first_modify_change(chain);

        if move_change.is_none() && modify_change.is_none() {
            return vec![];
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

            return vec![
                (move_change.0, optimized_move),
                (modify_change.0, optimized_modify),
            ];
        } else if move_change.is_none() && !modify_change.is_none() {
            let modify_change = modify_change.unwrap();
            return vec![modify_change];
        } else if !move_change.is_none() && modify_change.is_none() {
            let move_change = move_change.unwrap();

            let move_from = match &chain.front().unwrap().1 {
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

            return vec![(move_change.0, optimized_move)];
        }
    }
    let mut return_vec = vec![];
    for change in chain {
        return_vec.push(change.clone());
    }
    return return_vec;
}

/// Runtime of O(n*k) where k = the number of tables and n = the number of changes
/// This is based off of the merge-part of the merge-sort algorithm.
pub fn merge_changes(
    changes_in_table: Vec<Vec<(i32, data::ChangeEvent)>>,
) -> Vec<(i32, data::ChangeEvent)> {
    let mut iter_counts = vec![0; changes_in_table.len()];

    let mut changes = vec![];

    loop {
        let mut min_change_event_id = i32::MAX;
        let mut min_change_event_id_index = 0;

        for (i, iter_count) in iter_counts.iter().enumerate() {
            if *iter_count < changes_in_table[i].len() {
                let change_event_id = changes_in_table[i][*iter_count].0;

                if change_event_id < min_change_event_id {
                    min_change_event_id = change_event_id;
                    min_change_event_id_index = i;
                }
            } else {
                break;
            }
        }

        if min_change_event_id == i32::MAX {
            break;
        }

        changes.push(
            changes_in_table[min_change_event_id_index][iter_counts[min_change_event_id_index]]
                .clone(),
        );

        iter_counts[min_change_event_id_index] += 1;
    }

    changes
}

pub fn optimize_changes(changes: LinkedList<(i32, ChangeEvent)>) -> Vec<(i32, ChangeEvent)> {
    let (chains, deleted_chains) = get_chains(changes);

    let mut optimized_changes = vec![];

    for (_last_path, mut chain) in chains.into_iter() {
        let optimized = make_optimizations(&mut chain);
        optimized_changes.push(optimized);
    }

    for mut chain in deleted_chains.into_iter() {
        let optimized = make_optimizations(&mut chain);
        optimized_changes.push(optimized)
    }

    let mut merged = merge_changes(optimized_changes);

    merged.sort_by_key(|k| k.0);

    merged
}
