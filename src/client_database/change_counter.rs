use std::{fs, path};

/// Local change counter
/// Stores how many changes have been made to the local files.
/// Used primarily for creating unique file names.
#[derive(Debug, Clone)]
pub struct ChangeCounter {
    counter_path: path::PathBuf,
    change_count: i64,
    change_directory: path::PathBuf,
}

impl ChangeCounter {
    pub fn init(program_data_directory: &path::PathBuf) -> Self {
        let counter_path = program_data_directory.join("change_count");
        let change_count = if !counter_path.exists() {
            fs::write(&counter_path, "0").unwrap();
            0
        } else {
            fs::read_to_string(&counter_path)
                .unwrap()
                .parse::<i64>()
                .unwrap()
        };

        let change_dir = program_data_directory.join("changes");
        if !change_dir.exists() {
            fs::create_dir(&change_dir).unwrap();
        }

        Self {
            counter_path,
            change_count,
            change_directory: program_data_directory.join("changes"),
        }
    }

    pub fn increment(&mut self) -> i64 {
        self.change_count += 1;
        fs::write(&self.counter_path, self.change_count.to_string()).unwrap();
        self.change_count
    }

    pub fn next_path(&mut self) -> path::PathBuf {
        let change_count = self.increment();
        self.change_directory.join(format!("{}.tmp", change_count))
    }

    pub fn set_count(&mut self, count: i64) -> i64 {
        self.change_count = count;
        fs::write(&self.counter_path, self.change_count.to_string()).unwrap();
        self.change_count
    }

    pub fn change_count(&self) -> i64 {
        self.change_count
    }
}
