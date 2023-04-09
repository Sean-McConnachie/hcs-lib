extern crate toml;

use serde::Deserialize;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_config<P: AsRef<Path>, T: for<'de> Deserialize<'de>>(
    path: P,
) -> Result<T, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    toml::from_str(&contents).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

#[allow(unused)]
pub fn parse_log_filter<'de, D>(deserializer: D) -> Result<log::LevelFilter, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "trace" => Ok(log::LevelFilter::Trace),
        "debug" => Ok(log::LevelFilter::Debug),
        "info" => Ok(log::LevelFilter::Info),
        "warn" => Ok(log::LevelFilter::Warn),
        "error" => Ok(log::LevelFilter::Error),
        _ => Err(serde::de::Error::custom(format!(
            "Invalid log level: {}",
            s
        ))),
    }
}

#[allow(unused)]
pub fn parse_path_buf<'de, D>(deserializer: D) -> Result<std::path::PathBuf, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(std::path::PathBuf::from(s))
}
