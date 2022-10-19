use std::{
    fs::File,
    io::{self, Read, Write},
};

use gauc::config;

fn main() -> io::Result<()> {
    let mut config = String::new();
    match &mut File::open(config::CONFIG_PATH) {
        Ok(v) => match v.read_to_string(&mut config) {
            Ok(_v) => (),
            Err(e) => panic!("Couldn't open config file: {}", e),
        },
        Err(_e) => {
            let config = config::Config::make_default();
            println!("{:?}", config);
            config.write(config::CONFIG_PATH).unwrap();
        }
    };

    let mut config = match config::Config::deserialize(&config) {
        Ok(v) => v,
        Err(e) => panic!("toml deserialize error: {}", e),
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

    config
        .account
        .push(config::Account::new(email_input, name_input));

    config.write(config::CONFIG_PATH).unwrap();

    Ok(())
}
