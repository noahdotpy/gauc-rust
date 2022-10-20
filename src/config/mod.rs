use serde_derive::{Deserialize, Serialize};
use std::{fs::File, io::Write};

pub const CONFIG_PATH: &str = "./target/test-config/test.toml";

#[derive(Deserialize, Serialize, Debug)]
pub struct Account {
    email: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub account: Option<Vec<Account>>,
}

impl Config {
    pub fn make_default() -> Config {
        Config {
            account: Some(vec![Account {
                email: String::from("noahdotpy@github.com"),
                name: String::from("noahdotpy"),
            }]),
        }
    }
    /// Creates a new file at path if doesn't exist (doesn't create directories), serializes self (config object) and writes the contents of serialized config into that.
    pub fn write(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = std::path::Path::new(path);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();

        let toml = toml::to_string(self)?;

        let mut file = File::create(path)?;

        // Write a &str into the file
        writeln!(&mut file, "{toml}")?;
        Ok(())
    }
}
