use serde::{Deserialize, Serialize};
//use std::env;
//use std::f32::consts::E;
use std::fs;
use std::io;
//use std::path::Path;

const DEFAULT_SLIDE_WIDTH: i32 = 1400;
const DEFAULT_SLIDE_HEIGHT: i32 = 1050;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultValues {
    slide_width: i32,
    slide_height: i32,
}

impl Default for DefaultValues {
    fn default() -> Self {
        Self {
            slide_width: DEFAULT_SLIDE_WIDTH,
            slide_height: DEFAULT_SLIDE_HEIGHT,
        }
    }
}

impl DefaultValues {
    fn new() -> Self {
        DefaultValues {
            slide_width: DEFAULT_SLIDE_WIDTH,
            slide_height: DEFAULT_SLIDE_HEIGHT,
        }
    }

    fn from_config_file_or_default(config_file_path: &str) -> Result<DefaultValues, io::Error> {
        match fs::read_to_string(config_file_path) {
            Ok(content) => {
                match  toml::from_str(&content) {
                    Ok(dv) => Ok(dv),
                    Err(e) => {
                        Ok(DefaultValues::new())
                    }
                }
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    #[cfg(test)]
                    {
                        eprintln!("Config file not found. Creating default config file at: {}",
                            config_file_path);
                    }
                    #[cfg(not(test))]
                    {}
                    let default_values = DefaultValues::new();
                    let toml_str = toml::to_string(&default_values).unwrap();
                    match fs::write(config_file_path, toml_str) {
                        Ok(()) => return Ok(default_values),
                        Err(err) => {
                            #[cfg(test)]
                            {
                                eprintln!("Failed to write default config file: {}", err);
                            }
                            #[cfg(not(test))]
                            {}
                            return Ok(default_values)
                        }
                    }
                } else {
                    return Err(e);
                }
            }
        }
    }

    pub fn get_slide_width(&self) -> i32 {
        self.slide_width
    }

    pub fn get_slide_height(&self) -> i32 {
        self.slide_height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_file() {
        let config_file_path = "test_config.toml";
        fs::remove_file(config_file_path).ok(); // Ensure the file does not exist
        let result = DefaultValues::from_config_file_or_default(config_file_path);
        let del_result = fs::remove_file(config_file_path);
        assert!(del_result.is_ok());
        assert!(result.is_ok());
        let defaults = result.unwrap();
        assert_eq!(defaults.get_slide_width(), DEFAULT_SLIDE_WIDTH);
        assert_eq!(defaults.get_slide_height(), DEFAULT_SLIDE_HEIGHT);
    }

    #[test]
    fn test_valid_file() -> Result<(), String> {
        let config_file_path = "test_config2.toml";
        let defaults = DefaultValues{slide_width: 1000, slide_height:750};
        match fs::write(config_file_path, toml::to_string(&defaults).unwrap()) {
            Err(e) => Err(String::from("Error writing default values file")),
            Ok(()) => {
                let result = DefaultValues::from_config_file_or_default(config_file_path);
                let del_result = fs::remove_file(config_file_path);
                assert!(del_result.is_ok());
                assert!(result.is_ok());
                let defaults = result.unwrap();
                assert_eq!(defaults.get_slide_width(), 1000);
                assert_eq!(defaults.get_slide_height(), 750);
                Ok(())
            }
        }
    }            

    #[test]
    fn test_invalid_content() -> Result<(), String> {
        let config_file_path = "test_config3.toml";
        let bad_content = String::from("slide_width = 1000
slide_heigh");
        match fs::write(config_file_path, bad_content) {
            Err(e) => Err(String::from(format!("Error writing bad content to file: {}", e))),
            Ok(()) => {
                let result = DefaultValues::from_config_file_or_default(config_file_path);
                let del_result = fs::remove_file(config_file_path);
                assert!(del_result.is_ok());
                match result {
                    Ok(dv) => {
                        assert_eq!(dv.get_slide_width(), 1400);
                        assert_eq!(dv.get_slide_height(), 1050);
                        Ok(())
                    }
                    Err(e) => Err(e.to_string())
                }
            }
        }
    }
}
