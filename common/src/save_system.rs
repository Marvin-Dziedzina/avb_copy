use std::{
    fs, io,
    path::{Path, PathBuf},
};

use bevy::prelude::*;

use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

#[derive(Debug)]
pub struct SaveSystem;

impl SaveSystem {
    /// Save data to `path`.
    ///
    /// This function will create all path components and the file if not existent.
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// - it fails to create all path components or the file.
    /// - the data failed to get serialized.
    pub fn save_data<'a, P, T>(path: P, data: &T) -> Result<(), SaveSystemError>
    where
        P: Into<PathBuf>,
        T: Serialize,
    {
        Self::save(path.into(), data)
    }

    /// Load data from `path`.
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// - `path` not already exist.
    /// - the data failed to get deserialized.
    pub fn load_data<'a, P, T>(path: P) -> Result<T, SaveSystemError>
    where
        P: Into<PathBuf>,
        T: DeserializeOwned,
    {
        Self::load(path.into())
    }

    /// Save data to `path`.
    ///
    /// This function will create all path components and the file if not existent.
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// - it fails to create all path components or the file.
    /// - the data failed to get serialized.
    pub fn save<P, T>(path: P, data: &T) -> Result<(), SaveSystemError>
    where
        P: AsRef<Path>,
        T: Serialize,
    {
        let ser_data = toml::to_string_pretty(data)?;
        Self::write(path, ser_data.as_bytes())?;

        Ok(())
    }

    /// Load data from `path`.
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// - `path` not already exist.
    /// - the data failed to get deserialized.
    pub fn load<P, T>(path: P) -> Result<T, SaveSystemError>
    where
        P: AsRef<Path>,
        T: DeserializeOwned,
    {
        let bytes = Self::read(path)?;
        let de_data = toml::from_slice(&bytes)?;

        Ok(de_data)
    }

    /// Write a slice as entire contents of a file.
    ///
    /// Create the path if it does not exist.
    ///
    /// # Errors
    ///
    /// The function will return an error if path could not be created.
    pub fn write<P>(path: P, contents: &[u8]) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        // Remove the file name from the path.
        let p = path
            .as_ref()
            .parent()
            .expect("Path can not be empty or root");
        fs::create_dir_all(p)?;

        fs::write(path, contents)?;

        Ok(())
    }

    /// Reads the entire contents of a file into a bytes vector.
    ///
    /// # Errors
    ///
    /// This function will return an error if `path` not already exist.
    pub fn read<P>(path: P) -> io::Result<Vec<u8>>
    where
        P: AsRef<Path>,
    {
        fs::read(path)
    }
}

#[derive(Debug, Error)]
pub enum SaveSystemError {
    #[error("tomle serialisation error")]
    TomlSerError(#[from] toml::ser::Error),
    #[error("toml deserialisation error")]
    TomlDeError(#[from] toml::de::Error),
    #[error("io error")]
    IoError(#[from] io::Error),
}

#[cfg(test)]
mod save_system_test {
    use std::{fs, path::PathBuf};

    use serde::{Deserialize, Serialize};

    use crate::save_system::SaveSystem;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Data {
        value: u8,
    }

    enum Paths {
        Test1,
        Test2,
    }

    impl From<Paths> for PathBuf {
        fn from(value: Paths) -> Self {
            match value {
                Paths::Test1 => "./test1/file",
                Paths::Test2 => "./test2/file",
            }
            .into()
        }
    }

    #[test]
    fn write_read_test() {
        let path = "./test";
        let contents = b"Yay".to_vec();

        SaveSystem::write(&path, &contents).unwrap();
        let read_contents = SaveSystem::read(&path).unwrap();

        fs::remove_file(path).unwrap();

        assert_eq!(contents, read_contents)
    }

    #[test]
    fn write_read_large_path_test() {
        let path = "./testdir/test";
        let contents = b"Yay".to_vec();

        SaveSystem::write(&path, &contents).unwrap();
        let read_contents = SaveSystem::read(&path).unwrap();

        fs::remove_dir_all(PathBuf::from(path).parent().unwrap()).unwrap();

        assert_eq!(contents, read_contents)
    }

    #[test]
    fn save_load_test() {
        let path = "./testsaveload";
        let contents = Data { value: 101 };

        SaveSystem::save(&path, &contents).unwrap();
        let read_contents = SaveSystem::load(&path).unwrap();

        fs::remove_file(&path).unwrap();

        assert_eq!(contents, read_contents);
    }

    #[test]
    fn save_load_data_test() {
        let contents = Data { value: 3 };

        SaveSystem::save_data(Paths::Test1, &contents).unwrap();
        SaveSystem::save_data(Paths::Test2, &contents).unwrap();

        let read1_contents: Data = SaveSystem::load_data(Paths::Test1).unwrap();
        let read2_contents: Data = SaveSystem::load_data(Paths::Test2).unwrap();

        let path1 = PathBuf::from(Paths::Test1);
        let path1_parent = path1.parent().unwrap();
        let path2 = PathBuf::from(Paths::Test2);
        let path2_parent = path2.parent().unwrap();
        fs::remove_dir_all(path1_parent).unwrap();
        fs::remove_dir_all(path2_parent).unwrap();

        assert_eq!(contents, read1_contents);
        assert_eq!(contents, read2_contents);
    }
}
