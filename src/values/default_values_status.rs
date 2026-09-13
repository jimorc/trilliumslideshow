use std::fmt;
use std::fmt::Display;
use std::io;

/// An enum of status codes for reading and writing the default configuration file,
/// and if necessary, creating DefaultValues object.
///
/// A number of status codes may be specified when reading writing the configuration file.
/// For example, if the configuration file does not exist, then a new DefaultValues object
/// would be created, and that object would be serialized and written to a new configuration
/// file. In that instance, FileNotFound, NewDefaultsCreated, and FileWritten would all be
/// specified. This would allow a number of messages to be displayed to the program user to
/// let them know what has occured.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DefaultValuesStatus {
    /// The $HOME environment variable is not set.
    HomeNotSet,
    /// The configuration file has been read and converted to a DefaultValues object.
    FileRead,
    // An IO error other than file not found occured while trying to read the configuration
    // file.
    FileReadError(io::ErrorKind),
    /// The configuration file has been read, but its contents are invalid.
    InvalidConfig,
    /// The configuration file does not exist.
    FileNotFound,
    /// A new DefaultValues object was created.
    NewDefaultsCreated,
    /// The DefaultValues object was written to the configuration file.
    FileWritten,
    /// An IO error occured while tryiing to write the
    /// DefaultValues object to the configuration file.
    FileWriteError(io::ErrorKind),
    /// The directory containing the defaults file does not exist and cannot be created.
    CannotCreateConfigFolder(io::ErrorKind),
}

impl Display for DefaultValuesStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DefaultValuesStatus::HomeNotSet => {
                write!(
                    f,
                    "the HOME environment variable is not set. Cannot determine location of defaults file"
                )
            }
            DefaultValuesStatus::FileRead => {
                write!(
                    f,
                    "default values have been read from the configuration file"
                )
            }
            DefaultValuesStatus::FileReadError(kind) => {
                write!(
                    f,
                    "the error: {} occurred while attempting to read the configuration file",
                    kind
                )
            }
            DefaultValuesStatus::InvalidConfig => {
                write!(f, "the configuration file contained invalid data")
            }
            DefaultValuesStatus::FileNotFound => {
                write!(f, "the configuration file does not exist")
            }
            DefaultValuesStatus::NewDefaultsCreated => {
                write!(f, "a new default values object has been created")
            }
            DefaultValuesStatus::FileWritten => {
                write!(
                    f,
                    "the new default values have been written to the configuration file"
                )
            }
            DefaultValuesStatus::FileWriteError(kind) => {
                write!(
                    f,
                    "the error: {} occurred while attempting to write the configuration file",
                    kind
                )
            }
            DefaultValuesStatus::CannotCreateConfigFolder(kind) => {
                write!(
                    f,
                    "the error: {} occurred while attempting to create the directory for the
configuration file. Configuration file cannot be saved",
                    kind
                )
            } // ...other enumerations...
        }
    }
}

#[cfg(test)]
#[test]
fn no_home_test() {
    assert_eq!(
        "the HOME environment variable is not set. Cannot determine location of defaults file",
        DefaultValuesStatus::HomeNotSet.to_string()
    );
}

#[test]
fn file_read_test() {
    assert_eq!(
        "default values have been read from the configuration file",
        DefaultValuesStatus::FileRead.to_string()
    );
}

#[test]
fn file_read_error_test() {
    assert_eq!(
        "the error: connection refused occurred while attempting to read the configuration file",
        DefaultValuesStatus::FileReadError(io::ErrorKind::ConnectionRefused).to_string()
    );
}

#[test]
fn invalid_config_test() {
    assert_eq!(
        "the configuration file contained invalid data",
        DefaultValuesStatus::InvalidConfig.to_string()
    );
}

#[test]
fn file_not_found_test() {
    assert_eq!(
        "the configuration file does not exist",
        DefaultValuesStatus::FileNotFound.to_string()
    );
}

#[test]
fn new_defaults_created_test() {
    assert_eq!(
        "a new default values object has been created",
        DefaultValuesStatus::NewDefaultsCreated.to_string()
    );
}

#[test]
fn file_written_test() {
    assert_eq!(
        "the new default values have been written to the configuration file",
        DefaultValuesStatus::FileWritten.to_string()
    );
}

#[test]
fn file_write_error_test() {
    assert_eq!(
        "the error: connection refused occurred while attempting to write the configuration file",
        DefaultValuesStatus::FileWriteError(io::ErrorKind::ConnectionRefused).to_string()
    );
}

#[test]
fn cannot_create_config_dir_test() {
    assert_eq!(
        "the error: entity already exists occurred while attempting to create the directory for the\nconfiguration file. Configuration file cannot be saved",
        DefaultValuesStatus::CannotCreateConfigFolder(io::ErrorKind::AlreadyExists).to_string(),
    )
}
