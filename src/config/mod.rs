use serde::de::value::Error;
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
    // valid toml &str -> Config
    pub fn deserialize(data: &str) -> Result<Config, toml::de::Error> {
        toml::from_str(data)
    }
    // Config -> &str
    pub fn serialize(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(&self)
    }
    pub fn write(&self, path: String) -> Result<(), Error> {
        // TODO: I should check for errors before I unwrap and return them

        let toml = self.serialize();
        let toml = toml.unwrap();

        let file = File::create(path);
        let mut file = file.unwrap();

        // Write a &str into the file (ignoring the result).
        writeln!(&mut file, "{toml}");
        Ok(())
    }
}
