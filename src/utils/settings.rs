use macroquad::error;
use std::fs;
use std::io::Error;

// Default values in case there is no config file found.
const WIDTH: i32 = 40;
const HEIGHT: i32 = 40;

/// Split the key=value pair into tuple of strings
fn parse_pair(line: &str) -> (String, String) {
    let mut iter = line.splitn(2, '=');
    let key = iter.next().expect("parse_pair failed");
    let value = iter.next().expect("parse_pair failed");
    (key.to_string(), value.to_string())
}
struct ConfigFile {
    _path: String,
    vars: Vec<(String, String)>,
}

impl ConfigFile {
    fn new(path: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(path)?;
        let mut vars = Vec::with_capacity(2);
        for line in contents.lines() {
            vars.push(parse_pair(line));
        }
        Ok(Self {
            _path: path.to_string(),
            vars,
        })
    }
}

// Centralized struct for all customizable variables.
pub struct Settings {
    _config: Option<ConfigFile>,
    pub width: i32,
    pub height: i32,
}

impl Settings {
    /// Try to read settings file and populate `Settings` struct, provide defaults otherwise.
    pub fn init(path: &str) -> Self {
        match ConfigFile::new(path) {
            Ok(config) => {
                println!("Loading settings file.");
                Settings::parse_config(config)
            }
            Err(e) => {
                error!(
                    "Unable to load setting file at {}! Loading default settings. Error: {}",
                    path, e
                );
                Settings::default()
            }
        }
    }
    fn parse_config(config: ConfigFile) -> Self {
        let mut width = WIDTH;
        let mut height = HEIGHT;
        for (key, value) in &config.vars {
            match key.as_str() {
                "width" => {
                    width = value
                        .parse::<i32>()
                        .expect(format!("Cannot parse value {} in key {}!", value, key).as_str())
                }
                "height" => {
                    height = value
                        .parse::<i32>()
                        .expect(format!("Cannot parse value {} in key {}!", value, key).as_str())
                }
                _ => error!("unknown key {}", key),
            }
        }
        Self {
            _config: Some(config),
            width,
            height,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            _config: None,
            width: WIDTH,
            height: HEIGHT,
        }
    }
}
