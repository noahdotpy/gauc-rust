use serde_derive::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};
#[derive(Deserialize, Serialize, Debug)]
pub struct Account {
    email: String,
    name: String,
}

impl Account {
    pub fn new(email: String, name: String) -> Account {
        Account { email, name }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub accounts: Option<Vec<Account>>,
}

impl Config {
    pub fn new(accounts: Option<Vec<Account>>) -> Config {
        Config { accounts }
    }
    /// Returns a string slice representation of the configuration file
    pub fn parse_to_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = String::new();
        File::open(path)?.read_to_string(&mut result)?;
        Ok(result)
    }
    /// Returns a config struct representation of the configuration file
    pub fn parse_to_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let file = Config::parse_to_string(path)?;
        let result = Config::deserialize(&file).unwrap();
        Ok(result)
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
    pub fn write(&self, path: String) -> Result<(), Box<dyn std::error::Error>> {
        let toml = self.serialize()?;

        let mut file = File::create(path)?;

        // Write a &str into the file (ignoring the result).
        writeln!(&mut file, "{toml}")?;
        Ok(())
    }
}
