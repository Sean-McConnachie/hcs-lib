use std::path;

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]

/// A file unit struct.
pub struct FileType;
/// A directory unit struct.
pub struct DirectoryType;
/// A symlink unit struct.
pub struct SymlinkType;
/// A custom metadata unit struct.
pub struct CustomMetadataType;

/// A storage directory unit struct.
pub struct StorageDir;
/// A symlink directory unit struct.
pub struct SymlinkDir;
/// A unit struct for any other directory.
pub struct OtherDir;

impl SymlinkType {
    pub fn is_symlink_type(fp: &path::PathBuf) -> bool {
        fp.is_symlink()
    }

    pub fn points_to(fp: &path::PathBuf) -> path::PathBuf {
        let points_to = std::fs::read_link(fp).unwrap();
        points_to
    }

    pub fn to_real(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> path::PathBuf {
        let relative_path = fp.strip_prefix(&config.symlink_directory).unwrap();
        let real_path = config.storage_directory.join(relative_path);
        real_path
    }

    pub fn to_custom_metadata(
        fp: &path::PathBuf,
        config: &super::FileHandlerConfig,
    ) -> path::PathBuf {
        let real_path = SymlinkType::to_real(fp, config);
        let custom_metadata_path = FileType::to_custom_metadata(&real_path);
        custom_metadata_path
    }

    pub fn to_relative(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> path::PathBuf {
        let relative_path = fp.strip_prefix(&config.symlink_directory).unwrap();
        relative_path.to_path_buf()
    }
}

impl FileType {
    pub fn is_file_type(fp: &path::PathBuf) -> bool {
        fp.extension() != Some(std::ffi::OsStr::new("sc")) && fp.is_file()
    }

    pub fn to_custom_metadata(fp: &path::PathBuf) -> path::PathBuf {
        let parent_dir = fp.parent().unwrap();
        let custom_metadata_path =
            parent_dir.join(format!(".{}.sc", fp.file_name().unwrap().to_str().unwrap()));
        custom_metadata_path
    }

    pub fn to_symlink(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> path::PathBuf {
        let relative_path = fp.strip_prefix(&config.storage_directory).unwrap();
        let symlink_path = config.symlink_directory.join(relative_path);
        symlink_path
    }

    pub fn to_relative(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> path::PathBuf {
        let relative_path = fp.strip_prefix(&config.storage_directory).unwrap();
        relative_path.to_path_buf()
    }
}

impl DirectoryType {
    pub fn is_directory_type(fp: &path::PathBuf) -> bool {
        fp.is_dir()
    }

    pub fn to_custom_metadata(fp: &path::PathBuf) -> path::PathBuf {
        FileType::to_custom_metadata(fp)
    }

    pub fn to_symlink(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> path::PathBuf {
        FileType::to_symlink(fp, config)
    }

    pub fn to_relative(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> path::PathBuf {
        FileType::to_relative(fp, config)
    }
}

impl CustomMetadataType {
    pub fn is_custom_metadata(fp: &path::PathBuf) -> bool {
        fp.extension() == Some(std::ffi::OsStr::new("sc"))
    }

    pub fn to_real(fp: &path::PathBuf) -> path::PathBuf {
        let parent_dir = fp.parent().unwrap();
        let file_name = fp.file_name().unwrap().to_str().unwrap()[1..]
            .strip_suffix(".sc")
            .unwrap();
        let normal_file = parent_dir.join(file_name);
        normal_file
    }

    pub fn to_symlink(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> path::PathBuf {
        let real_path = CustomMetadataType::to_real(fp);
        let symlink_path = FileType::to_symlink(&real_path, config);
        symlink_path
    }

    pub fn to_relative(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> path::PathBuf {
        let real_path = CustomMetadataType::to_real(fp);
        let relative_path = real_path.strip_prefix(&config.storage_directory).unwrap();
        relative_path.to_path_buf()
    }
}

impl StorageDir {
    pub fn is_storage_dir(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> bool {
        fp == &config.storage_directory
    }

    pub fn in_storage_dir(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> bool {
        fp.starts_with(&config.storage_directory)
    }

    pub fn to_relative(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> path::PathBuf {
        let relative_path = fp.strip_prefix(&config.storage_directory).unwrap();
        relative_path.to_path_buf()
    }
}

impl SymlinkDir {
    pub fn is_symlink_dir(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> bool {
        fp == &config.symlink_directory
    }

    pub fn in_symlink_dir(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> bool {
        fp.starts_with(&config.symlink_directory)
    }

    pub fn to_relative(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> path::PathBuf {
        let relative_path = fp.strip_prefix(&config.symlink_directory).unwrap();
        relative_path.to_path_buf()
    }
}

impl OtherDir {
    pub fn is_other_dir(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> bool {
        !StorageDir::in_storage_dir(fp, config) && !SymlinkDir::in_symlink_dir(fp, config)
    }
}

pub fn relative_to_real(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> path::PathBuf {
    let real_path = config.storage_directory.join(fp);
    real_path
}

pub fn relative_to_custom_metadata(
    fp: &path::PathBuf,
    config: &super::FileHandlerConfig,
) -> path::PathBuf {
    let real_path = relative_to_real(fp, config);
    let custom_metadata_path = FileType::to_custom_metadata(&real_path);
    custom_metadata_path
}

pub fn relative_to_symlink(fp: &path::PathBuf, config: &super::FileHandlerConfig) -> path::PathBuf {
    let real_path = relative_to_real(fp, config);
    let symlink_path = FileType::to_symlink(&real_path, config);
    symlink_path
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum FileLocation {
    StorageDir,
    SymlinkDir,
    OtherDir,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Type {
    File,
    Directory,
    CustomMetadata,
    Symlink,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct FilePaths {
    path: path::PathBuf,
    storage_dir_path: path::PathBuf,
    symlink_dir_path: path::PathBuf,
    custom_metadata_path: path::PathBuf,
    relative_path: path::PathBuf,
    points_to: Option<path::PathBuf>,
    file_location: FileLocation,
    file_type: Type,
}

impl FilePaths {
    pub fn new(
        relative_path: path::PathBuf,
        config: &super::FileHandlerConfig,
        file_type: Type,
        file_location: FileLocation,
        points_to: Option<path::PathBuf>,
    ) -> Self {
        let storage_dir_path = relative_to_real(&relative_path, config);
        let symlink_dir_path = relative_to_symlink(&relative_path, config);
        let custom_metadata_path = relative_to_custom_metadata(&relative_path, config);

        FilePaths {
            path: relative_path.clone(),
            storage_dir_path,
            symlink_dir_path,
            custom_metadata_path,
            relative_path,
            points_to,
            file_location,
            file_type,
        }
    }

    pub fn from_path_checked(
        fp: path::PathBuf,
        config: &super::FileHandlerConfig,
    ) -> Result<Self, std::io::Error> {
        let (file_location, relative_path) = if StorageDir::in_storage_dir(&fp, config)
            && CustomMetadataType::is_custom_metadata(&fp)
        {
            (
                FileLocation::StorageDir,
                CustomMetadataType::to_relative(&fp, config),
            )
        } else if StorageDir::in_storage_dir(&fp, config) {
            (
                FileLocation::StorageDir,
                StorageDir::to_relative(&fp, config),
            )
        } else if SymlinkDir::in_symlink_dir(&fp, config) {
            (
                FileLocation::SymlinkDir,
                SymlinkDir::to_relative(&fp, config),
            )
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File is not in storage or symlink directory",
            ));
        };

        let file_type = if CustomMetadataType::is_custom_metadata(&fp) {
            Type::CustomMetadata
        } else if SymlinkType::is_symlink_type(&fp) {
            Type::Symlink
        } else if FileType::is_file_type(&fp) {
            Type::File
        } else if DirectoryType::is_directory_type(&fp) {
            Type::Directory
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File is not a file, directory, custom metadata, or symlink",
            ));
        };

        let file_paths = if matches!(file_type, Type::Symlink) {
            FilePaths::new(
                relative_path,
                config,
                Type::Symlink,
                file_location,
                Some(SymlinkType::points_to(&fp)),
            )
        } else {
            FilePaths::new(relative_path, config, file_type, file_location, None)
        };
        Ok(file_paths)
    }

    pub fn from_relative_path(
        relative_path: path::PathBuf,
        file_type: Type,
        file_location: FileLocation,
        points_to: Option<path::PathBuf>,
        config: &super::FileHandlerConfig,
    ) -> Result<Self, std::io::Error> {
        let file_paths = FilePaths {
            path: relative_path.clone(),
            storage_dir_path: relative_to_real(&relative_path, config),
            symlink_dir_path: relative_to_symlink(&relative_path, config),
            custom_metadata_path: relative_to_custom_metadata(&relative_path, config),
            relative_path,
            points_to,
            file_location,
            file_type,
        };
        Ok(file_paths)
    }

    pub fn path(&self) -> &path::PathBuf {
        &self.path
    }

    pub fn storage_dir_path(&self) -> &path::PathBuf {
        &self.storage_dir_path
    }

    pub fn symlink_dir_path(&self) -> &path::PathBuf {
        &self.symlink_dir_path
    }

    pub fn custom_metadata_path(&self) -> &path::PathBuf {
        &self.custom_metadata_path
    }

    pub fn relative_path(&self) -> &path::PathBuf {
        &self.relative_path
    }

    pub fn points_to(&self) -> Result<&path::PathBuf, std::io::Error> {
        // TODO: Make points_to check on function call
        match &self.points_to {
            Some(p) => Ok(p),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File is not a symlink. Does not have a points to path.",
            )),
        }
    }

    pub fn update_points_to(&mut self, points_to: path::PathBuf) {
        self.points_to = Some(points_to);
    }

    pub fn file_location(&self) -> &FileLocation {
        &self.file_location
    }

    pub fn file_type(&self) -> &Type {
        &self.file_type
    }
}

#[cfg(test)]
mod tests {
    use super::super::FileHandlerConfig;
    use super::{FileLocation, FilePaths, Type};
    use std::path;

    #[test]
    fn test_file_paths() {
        let mut config = FileHandlerConfig::default();
        config.storage_directory = path::PathBuf::from("_storage_dir");
        config.symlink_directory = path::PathBuf::from("_symlink_dir");

        let real_path = path::PathBuf::from("_storage_dir/dir1/real_file");
        let symlink_path = path::PathBuf::from("_symlink_dir/dir1/real_file");
        let custom_metadata_path = path::PathBuf::from("_storage_dir/dir1/.real_file.sc");
        let relative_path = path::PathBuf::from("dir1/real_file");

        let file_paths = FilePaths::new(
            relative_path.clone(),
            &config,
            Type::File,
            FileLocation::StorageDir,
            None,
        );
        assert_eq!(file_paths.storage_dir_path(), &real_path);
        assert_eq!(file_paths.symlink_dir_path(), &symlink_path);
        assert_eq!(file_paths.custom_metadata_path(), &custom_metadata_path);
        assert_eq!(file_paths.relative_path(), &relative_path);
        assert_eq!(file_paths.file_location(), &FileLocation::StorageDir);
        assert_eq!(file_paths.file_type(), &Type::File);
        assert_eq!(file_paths.points_to().is_err(), true);

        let points_to = path::PathBuf::from("points_to");
        let file_paths = FilePaths::new(
            relative_path.clone(),
            &config,
            Type::File,
            FileLocation::StorageDir,
            Some(points_to.clone()),
        );
        assert_eq!(file_paths.storage_dir_path(), &real_path);
        assert_eq!(file_paths.symlink_dir_path(), &symlink_path);
        assert_eq!(file_paths.custom_metadata_path(), &custom_metadata_path);
        assert_eq!(file_paths.relative_path(), &relative_path);
        assert_eq!(file_paths.file_location(), &FileLocation::StorageDir);
        assert_eq!(file_paths.file_type(), &Type::File);
        assert_eq!(file_paths.points_to().unwrap(), &points_to);
    }
}
