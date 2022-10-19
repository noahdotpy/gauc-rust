use std::{
    fs::File,
    io::{self, Read, Write},
};

use gauc::config::{Account, Config};

fn main() -> io::Result<()> {
    const CONFIG_PATH: &str = "test.toml";

    let mut config = String::new();
    match &mut File::open(CONFIG_PATH) {
        Ok(v) => match v.read_to_string(&mut config) {
            Ok(v) => v,
            Err(_) => panic!("Couldn't open config file: {}", CONFIG_PATH),
        },
        Err(_) => panic!("Couldn't open config file: {}", CONFIG_PATH),
    };

    let mut config = match Config::deserialize(&config) {
        Ok(v) => v,
        Err(_) => {
            let config = Config::new(vec![]);
            match config.write(CONFIG_PATH) {
                Ok(_) => (),
                Err(_) => panic!("Couldn't create new config file: {}", CONFIG_PATH),
            };

            config
        }
    };

    let mut email_input = String::new();
    print!("Email: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email_input).unwrap();
    let email_input = email_input.trim();

    let mut name_input = String::new();
    print!("Name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name_input).unwrap();
    let name_input = name_input.trim();

    config.account.push(Account::new(email_input, name_input));

    config.write(CONFIG_PATH).unwrap();

    Ok(())
}
