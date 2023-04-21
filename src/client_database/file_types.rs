use std::path;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum FileType {
    Real,
    CustomMetadata,
    Symlink,
    RealInSymlinkDir,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FilePaths {
    real_path: path::PathBuf,
    custom_metadata_path: path::PathBuf,
    symlink_path: path::PathBuf,
    relative_path: path::PathBuf,
    current_type: FileType,
}

impl FilePaths {
    pub fn from_path(
        fp: path::PathBuf,
        config: &super::FileHandlerConfig,
    ) -> Result<Self, std::io::Error> {
        let file_paths = if FileType::is_custom_metadata(&fp, config) {
            FilePaths {
                real_path: FileType::custom_metadata_to_real(&fp),
                symlink_path: FileType::custom_metadata_to_symlink(&fp, config),
                relative_path: FileType::custom_metadata_to_relative(&fp, config),
                custom_metadata_path: fp,
                current_type: FileType::CustomMetadata,
            }
        } else if FileType::is_real(&fp, config) {
            FilePaths {
                custom_metadata_path: FileType::real_to_custom_metadata(&fp),
                symlink_path: FileType::real_to_symlink(&fp, config),
                relative_path: FileType::real_to_relative(&fp, config),
                real_path: fp,
                current_type: FileType::Real,
            }
        } else if FileType::is_symlink(&fp, config) {
            FilePaths {
                real_path: FileType::symlink_to_real(&fp, config),
                custom_metadata_path: FileType::symlink_to_custom_metadata(&fp, config),
                relative_path: FileType::symlink_to_relative(&fp, config),
                symlink_path: fp,
                current_type: FileType::Symlink,
            }
        } else if FileType::is_in_symlink_directory(&fp, config) {
            // TODO: Possibly move above is_real?
            // TODO: Add directory filetype.
            FilePaths {
                real_path: FileType::symlink_to_real(&fp, config),
                custom_metadata_path: FileType::symlink_to_custom_metadata(&fp, config),
                relative_path: FileType::symlink_to_relative(&fp, config),
                symlink_path: fp,
                current_type: FileType::RealInSymlinkDir,
            }
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File is not in storage or symlink directory",
            ));
        };

        Ok(file_paths)
    }

    pub fn from_relative_path(
        fp: path::PathBuf,
        config: &super::FileHandlerConfig,
    ) -> Result<Self, std::io::Error> {
        let file_paths = FilePaths {
            real_path: FileType::relative_to_real(&fp, config),
            custom_metadata_path: FileType::relative_to_custom_metadata(&fp, config),
            symlink_path: FileType::relative_to_symlink(&fp, config),
            relative_path: fp,
            current_type: FileType::Real,
        };

        Ok(file_paths)
    }

    pub fn path(&self) -> &path::PathBuf {
        match self.current_type {
            FileType::Real => &self.real_path,
            FileType::CustomMetadata => &self.custom_metadata_path,
            FileType::Symlink => &self.symlink_path,
            FileType::RealInSymlinkDir => &self.symlink_path,
        }
    }

    pub fn real_path(&self) -> &path::PathBuf {
        &self.real_path
    }

    pub fn custom_metadata_path(&self) -> &path::PathBuf {
        &self.custom_metadata_path
    }

    pub fn symlink_path(&self) -> &path::PathBuf {
        &self.symlink_path
    }

    pub fn relative_path(&self) -> &path::PathBuf {
        &self.relative_path
    }

    pub fn current_type(&self) -> &FileType {
        &self.current_type
    }
}

impl FileType {
    pub fn real_to_custom_metadata(fp: &path::PathBuf) -> path::PathBuf {
        let parent_dir = fp.parent().unwrap();
        let custom_metadata_path =
            parent_dir.join(format!(".{}.sc", fp.file_name().unwrap().to_str().unwrap()));
        custom_metadata_path
    }

    pub fn real_to_symlink(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> path::PathBuf {
        let relative_path = fp.strip_prefix(&config.storage_directory).unwrap();
        let symlink_path = config.symlink_directory.join(relative_path);
        symlink_path
    }

    pub fn real_to_relative(
        fp: &path::PathBuf,
        config: &super::FileHandlerConfig,
    ) -> path::PathBuf {
        let relative_path = fp.strip_prefix(&config.storage_directory).unwrap();
        relative_path.to_path_buf()
    }

    pub fn custom_metadata_to_real(fp: &path::PathBuf) -> path::PathBuf {
        let parent_dir = fp.parent().unwrap();
        let file_name = fp.file_name().unwrap().to_str().unwrap()[1..]
            .strip_suffix(".sc")
            .unwrap();
        let normal_file = parent_dir.join(file_name);
        normal_file
    }

    pub fn custom_metadata_to_symlink(
        fp: &path::PathBuf,
        config: &super::FileHandlerConfig,
    ) -> path::PathBuf {
        let real_path = FileType::custom_metadata_to_real(fp);
        let symlink_path = FileType::real_to_symlink(&real_path, config);
        symlink_path
    }

    pub fn custom_metadata_to_relative(
        fp: &path::PathBuf,
        config: &super::FileHandlerConfig,
    ) -> path::PathBuf {
        let real_path = FileType::custom_metadata_to_real(fp);
        let relative_path = real_path.strip_prefix(&config.storage_directory).unwrap();
        relative_path.to_path_buf()
    }

    pub fn symlink_to_real(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> path::PathBuf {
        let relative_path = fp.strip_prefix(&config.symlink_directory).unwrap();
        let real_path = config.storage_directory.join(relative_path);
        real_path
    }

    pub fn symlink_to_custom_metadata(
        fp: &path::PathBuf,
        config: &super::FileHandlerConfig,
    ) -> path::PathBuf {
        let real_path = FileType::symlink_to_real(fp, config);
        let custom_metadata_path = FileType::real_to_custom_metadata(&real_path);
        custom_metadata_path
    }

    pub fn symlink_to_relative(
        fp: &path::PathBuf,
        config: &super::FileHandlerConfig,
    ) -> path::PathBuf {
        let relative_path = fp.strip_prefix(&config.symlink_directory).unwrap();
        relative_path.to_path_buf()
    }

    pub fn relative_to_real(
        fp: &path::PathBuf,
        config: &super::FileHandlerConfig,
    ) -> path::PathBuf {
        let real_path = config.storage_directory.join(fp);
        real_path
    }

    pub fn relative_to_custom_metadata(
        fp: &path::PathBuf,
        config: &super::FileHandlerConfig,
    ) -> path::PathBuf {
        let real_path = FileType::relative_to_real(fp, config);
        let custom_metadata_path = FileType::real_to_custom_metadata(&real_path);
        custom_metadata_path
    }

    pub fn relative_to_symlink(
        fp: &path::PathBuf,
        config: &super::FileHandlerConfig,
    ) -> path::PathBuf {
        let real_path = FileType::relative_to_real(fp, config);
        let symlink_path = FileType::real_to_symlink(&real_path, config);
        symlink_path
    }

    pub fn is_custom_metadata(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> bool {
        fp.extension() == Some(std::ffi::OsStr::new("sc"))
            && fp.starts_with(&config.storage_directory)
    }

    pub fn is_real(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> bool {
        fp.extension() != Some(std::ffi::OsStr::new("sc"))
            && fp.starts_with(&config.storage_directory)
    }

    pub fn is_symlink(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> bool {
        fp.starts_with(&config.symlink_directory) && fp.is_symlink()
    }

    pub fn is_in_symlink_directory(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> bool {
        fp.starts_with(&config.symlink_directory)
    }
}

#[cfg(test)]
mod tests {
    use super::super::FileHandlerConfig;
    use super::FilePaths;
    use std::path;

    #[test]
    fn test_file_paths() {
        let mut config = FileHandlerConfig::default();
        config.storage_directory = path::PathBuf::from("storage");
        config.symlink_directory = path::PathBuf::from("symlink");

        let real_path = path::PathBuf::from("storage/dir1/real_file");
        let symlink_path = path::PathBuf::from("symlink/dir1/real_file");
        let custom_metadata_path = path::PathBuf::from("storage/dir1/.real_file.sc");
        let relative_path = path::PathBuf::from("dir1/real_file");

        let file_paths = FilePaths::from_path(real_path.clone(), &config).unwrap();
        assert_eq!(file_paths.real_path(), &real_path);
        assert_eq!(file_paths.symlink_path(), &symlink_path);
        assert_eq!(file_paths.custom_metadata_path(), &custom_metadata_path);
        assert_eq!(file_paths.relative_path(), &relative_path);

        let file_paths = FilePaths::from_path(symlink_path.clone(), &config).unwrap();
        assert_eq!(file_paths.real_path(), &real_path);
        assert_eq!(file_paths.symlink_path(), &symlink_path);
        assert_eq!(file_paths.custom_metadata_path(), &custom_metadata_path);
        assert_eq!(file_paths.relative_path(), &relative_path);

        let file_paths = FilePaths::from_path(custom_metadata_path.clone(), &config).unwrap();
        assert_eq!(file_paths.real_path(), &real_path);
        assert_eq!(file_paths.symlink_path(), &symlink_path);
        assert_eq!(file_paths.custom_metadata_path(), &custom_metadata_path);
        assert_eq!(file_paths.relative_path(), &relative_path);

        let file_paths = FilePaths::from_relative_path(relative_path.clone(), &config).unwrap();
        assert_eq!(file_paths.real_path(), &real_path);
        assert_eq!(file_paths.symlink_path(), &symlink_path);
        assert_eq!(file_paths.custom_metadata_path(), &custom_metadata_path);
        assert_eq!(file_paths.relative_path(), &relative_path);
    }
}
