use crate::values::default_values_status::DefaultValuesStatus;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
#[cfg(test)]
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

const DEFAULT_SLIDE_WIDTH: i32 = 1400;
const DEFAULT_SLIDE_HEIGHT: i32 = 1050;

/// DataValues contains the default values for various parameters used in
/// tssconfigurator.
///
/// tssconfigurator provides one or more ways to change these defaults.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct DataValues {
    slide_width: i32,
    slide_height: i32,
}

impl DataValues {
    /// new creates a new DataValues object with standard default values set.
    pub fn new() -> Self {
        Self {
            slide_width: DEFAULT_SLIDE_WIDTH,
            slide_height: DEFAULT_SLIDE_HEIGHT,
        }
    }

    fn get_config_path() -> Option<PathBuf> {
        let path = env::home_dir()?;
        let config_path: PathBuf = [
            path.to_str().unwrap(),
            ".config",
            "trilliumslideshow",
            "defaults.toml",
        ]
        .iter()
        .collect();
        Some(config_path)
    }

    /// Create a DefaultValues object as follows:
    ///
    /// 1. From the contents of the configuration file if this is valid.
    /// 2. Using DefaultValues::new() if the $HOME environment values is not set,
    ///    file does not exist, cannot be read, or the
    ///    file's contents cannot be parsed to a DefaultValues object. In any of
    ///    these cases, an attempt is made to write the new contents to the configuration
    ///    file.
    ///
    /// # Test Limitations:
    /// This function checks the value of the HOME environment variable. Because
    /// UNIX-like systems have system-level restrictions that prevent env::set_env
    /// from changing the HOME variable, it is very difficult to test this function.
    pub fn from_config_file_if_exists() -> (DataValues, Vec<DefaultValuesStatus>) {
        let config_path = DataValues::get_config_path().unwrap();
        let path = config_path.to_str().unwrap();
        if path.is_empty() {
            DataValues::no_home()
        } else {
            DataValues::from_config_file_or_default(path)
        }
    }

    fn no_home() -> (DataValues, Vec<DefaultValuesStatus>) {
        let mut statuses = Vec::<DefaultValuesStatus>::new();
        statuses.push(DefaultValuesStatus::HomeNotSet);
        let dv = DataValues::new();
        statuses.push(DefaultValuesStatus::NewDefaultsCreated);
        (dv, statuses)
    }

    fn from_config_file_or_default(
        config_file_path: &str,
    ) -> (DataValues, Vec<DefaultValuesStatus>) {
        let mut statuses = Vec::<DefaultValuesStatus>::new();
        match fs::read_to_string(config_file_path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(dv) => {
                    statuses.push(DefaultValuesStatus::FileRead);
                    (dv, statuses)
                }
                Err(_) => {
                    statuses.push(DefaultValuesStatus::InvalidConfig);
                    let dv = DataValues::new();
                    statuses.push(DefaultValuesStatus::NewDefaultsCreated);
                    let sts = dv.write_defaults(config_file_path);
                    for s in sts {
                        statuses.push(s);
                    }
                    (dv, statuses)
                }
            },
            Err(read_error) => {
                if read_error.kind() == io::ErrorKind::NotFound {
                    statuses.push(DefaultValuesStatus::FileNotFound);
                    let dv = DataValues::new();
                    statuses.push(DefaultValuesStatus::NewDefaultsCreated);
                    let sts = dv.write_defaults(config_file_path);
                    for s in sts {
                        statuses.push(s);
                    }
                    (dv, statuses)
                } else {
                    statuses.push(DefaultValuesStatus::FileReadError(read_error.kind()));
                    let dv = DataValues::new();
                    statuses.push(DefaultValuesStatus::NewDefaultsCreated);
                    let sts = dv.write_defaults(config_file_path);
                    for s in sts {
                        statuses.push(s);
                    }
                    (dv, statuses)
                }
            }
        }
    }

    /// Retrieve the slide width.
    pub fn get_slide_width(&self) -> i32 {
        self.slide_width
    }

    /// Retrieve the slide height.
    pub fn get_slide_height(&self) -> i32 {
        self.slide_height
    }

    fn write_defaults(&self, config_file_path: &str) -> Vec<DefaultValuesStatus> {
        let mut statuses = Vec::<DefaultValuesStatus>::new();
        // the following line should never panic.
        // If it does, there is a bug in the code, probably in DefaultValues::new()
        let toml_str = toml::to_string(&self).unwrap();
        let toml_str1 = toml_str.clone();

        match fs::write(config_file_path, toml_str) {
            Ok(()) => {
                statuses.push(DefaultValuesStatus::FileWritten);
                statuses
            }
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    let dir = Path::new(config_file_path).parent().unwrap();
                    match fs::create_dir_all(dir) {
                        Ok(_) => {
                            // do nothing
                        }
                        Err(e) => {
                            statuses.push(DefaultValuesStatus::CannotCreateConfigFolder(e.kind()));
                            return statuses;
                        }
                    }
                    match fs::write(config_file_path, toml_str1) {
                        Ok(_) => {
                            statuses.push(DefaultValuesStatus::FileWritten);
                            return statuses;
                        }
                        Err(e) => {
                            statuses.push(DefaultValuesStatus::FileWriteError(e.kind()));
                            return statuses;
                        }
                    }
                }
                statuses.push(DefaultValuesStatus::FileWriteError(err.kind()));
                statuses
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_file() {
        let config_file_path = "test_config.toml";
        fs::remove_file(config_file_path).ok(); // Ensure the file does not exist
        let (dv, statuses) = DataValues::from_config_file_or_default(config_file_path);
        let del_result = fs::remove_file(config_file_path);
        assert!(del_result.is_ok());
        assert_eq!(DataValues::new(), dv);
        let stats = vec![
            DefaultValuesStatus::FileNotFound,
            DefaultValuesStatus::NewDefaultsCreated,
            DefaultValuesStatus::FileWritten,
        ];
        assert_eq!(stats, statuses);
    }

    #[test]
    fn test_valid_file() -> Result<(), String> {
        let config_file_path = "test_config2.toml";
        let defaults = DataValues {
            slide_width: 1000,
            slide_height: 750,
        };
        match fs::write(config_file_path, toml::to_string(&defaults).unwrap()) {
            Err(_) => Err(String::from("Error writing default values file")),
            Ok(()) => {
                let (dv, statuses) = DataValues::from_config_file_or_default(config_file_path);
                let del_result = fs::remove_file(config_file_path);
                assert!(del_result.is_ok());
                assert_eq!(defaults, dv);
                let stats = vec![DefaultValuesStatus::FileRead];
                assert_eq!(stats, statuses);
                Ok(())
            }
        }
    }

    #[test]
    fn test_invalid_content() -> Result<(), String> {
        let config_file_path = "test_config3.toml";
        let bad_content = String::from(
            "slide_width = 1000
    slide_heigh",
        );
        match fs::write(config_file_path, bad_content) {
            Err(e) => Err(format!("Error writing bad content to file: {}", e)),
            Ok(()) => {
                let (dv, statuses) = DataValues::from_config_file_or_default(config_file_path);
                let del_result = fs::remove_file(config_file_path);
                assert!(del_result.is_ok());
                assert_eq!(DataValues::new(), dv);
                let stats = vec![
                    DefaultValuesStatus::InvalidConfig,
                    DefaultValuesStatus::NewDefaultsCreated,
                    DefaultValuesStatus::FileWritten,
                ];
                assert_eq!(stats, statuses);

                Ok(())
            }
        }
    }

    #[test]
    fn test_bad_write_defaults() {
        let config_file_path = "test_config4.toml";
        let dv = DataValues::new();

        let exists = fs::exists(config_file_path);
        match exists {
            Ok(true) => {
                fs::remove_file(config_file_path).unwrap();
            }
            Ok(false) => {}
            Err(_) => {}
        }
        let file = File::create(config_file_path).unwrap();
        let mut permissions = file.metadata().unwrap().permissions();
        permissions.set_readonly(true);
        fs::set_permissions(config_file_path, permissions).unwrap();

        let status = dv.write_defaults(config_file_path);
        let mut permissions2 = file.metadata().unwrap().permissions();

        #[allow(clippy::permissions_set_readonly_false)]
        permissions2.set_readonly(false);

        #[allow(clippy::single_match)]
        match fs::set_permissions(config_file_path, permissions2) {
            // just ignore
            Ok(_) => {}
            Err(_) => {}
        }

        fs::remove_file(config_file_path).unwrap();
        let stats = vec![DefaultValuesStatus::FileWriteError(
            io::ErrorKind::PermissionDenied,
        )];
        assert_eq!(stats, status);
    }

    #[test]
    fn test_cannot_write_cannot_create_config_dir() {
        #[allow(unused_assignments)]
        let mut config_file_path = "";
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            config_file_path = "/etc/trilliumslideshow/test_config4.toml";
        }
        #[cfg(target_os = "windows")]
        {
            unreachable!("Windows code has not yet been written!";)
        }
        let dv = DataValues::new();
        let statuses = dv.write_defaults(config_file_path);
        let stats: Vec<DefaultValuesStatus> = vec![DefaultValuesStatus::CannotCreateConfigFolder(
            io::ErrorKind::PermissionDenied,
        )];
        assert_eq!(stats, statuses);
    }

    #[test]
    fn test_no_home() {
        let (dv, statuses) = DataValues::no_home();
        assert_eq!(DataValues::new(), dv);
        let stats = vec![
            DefaultValuesStatus::HomeNotSet,
            DefaultValuesStatus::NewDefaultsCreated,
        ];
        assert_eq!(stats, statuses);
    }
}
