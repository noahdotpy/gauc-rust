use serde_derive::{Deserialize, Serialize};
use std::{fs::File, io::Write};

pub const CONFIG_PATH: &str = "./beepboop/test.toml";

#[derive(Deserialize, Serialize, Debug)]
pub struct Account {
    email: String,
    name: String,
}

impl Account {
    pub fn new(email: &str, name: &str) -> Account {
        Account {
            email: email.to_string(),
            name: name.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub account: Vec<Account>,
}

impl Config {
    pub fn make_default() -> Config {
        Config::new(vec![])
    }
    /// Create a new Config with function args and return
    pub fn new(account: Vec<Account>) -> Config {
        Config { account }
    }
    /// Deserializes a toml string into a Config object.
    pub fn deserialize(data: &str) -> Result<Config, toml::de::Error> {
        toml::from_str(data)
    }
    /// Serializes a Config object into a string representation.
    pub fn serialize(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(&self)
    }
    /// Creates a new file at path if doesn't exist (doesn't create directories), serializes self (config object) and writes the contents of serialized config into that.
    pub fn write(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = std::path::Path::new(path);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();

        let toml = self.serialize()?;

        let mut file = File::create(path)?;

        // Write a &str into the file
        writeln!(&mut file, "{toml}")?;
        Ok(())
    }
}
