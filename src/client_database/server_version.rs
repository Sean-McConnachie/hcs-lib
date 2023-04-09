use std::{fs, path};

/// Local server version
pub struct ServerVersion {
    server_version_path: path::PathBuf,
    server_version: i64,
}

impl ServerVersion {
    pub fn init(program_data_directory: &path::PathBuf) -> Self {
        let server_version_path = program_data_directory.join("server_version");
        let server_version = if !server_version_path.exists() {
            fs::write(&server_version_path, "0").unwrap();
            0
        } else {
            fs::read_to_string(&server_version_path)
                .unwrap()
                .parse::<i64>()
                .unwrap()
        };
        Self {
            server_version_path,
            server_version,
        }
    }

    pub fn set(&mut self, new_server_version: i64) -> i64 {
        fs::write(&self.server_version_path, new_server_version.to_string()).unwrap();
        self.server_version = new_server_version;
        self.server_version
    }

    pub fn server_version(&self) -> i64 {
        self.server_version
    }
}
