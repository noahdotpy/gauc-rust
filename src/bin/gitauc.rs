use std::{
    fs::File,
    io::{self, Read},
};

use gitauc::config;

fn main() -> io::Result<()> {
    let mut config = String::new();
    let config = match File::open(config::CONFIG_PATH) {
        Ok(mut v) => match v.read_to_string(&mut config) {
            Ok(_) => {
                println!("Found file and loading");
                match toml::from_str(&config) {
                    Ok(v) => v,
                    Err(e) => panic!("toml deserialize error: {}", e),
                }
            }
            Err(e) => panic!("Couldn't open config file: {}", e),
        },
        Err(_e) => {
            println!("Config file not found, making new");
            let config = config::Config::make_default();
            config.write(config::CONFIG_PATH).unwrap();
            config
        }
    };

    println!("{:?}", config);

    Ok(())
}
