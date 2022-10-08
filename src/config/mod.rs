use serde_derive::{Deserialize, Serialize};
use std::{fs::File, io::Write};

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
    accounts: Option<Vec<Account>>,
}

impl Config {
    pub fn new(accounts: Option<Vec<Account>>) -> Config {
        Config { accounts }
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
