use dirs;
use serde_derive::Deserialize;
use std::env::consts::OS;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub header: Vec<Vec<String>>,
    pub home: Vec<Vec<String>>,
}

pub fn read_config() -> Config {
    // Making default empty header and home page.
    let config = Config {
        header: vec![vec![String::from("")]],
        home: vec![vec![String::from("")]],
    };

    // This variable for storing config file.
    let file: Result<String, std::io::Error>;
    let home_dir_addr = dirs::home_dir(); // Home directory address.
    if home_dir_addr.is_some() {
        // Trying to open and read config file in windows and other operating systems.
        if OS == "windows" {
            file = std::fs::read_to_string(format!(
                "{}{}",
                home_dir_addr.unwrap().display(),
                "\\AppData\\Local\\rlog\\rlog.toml"
            ));
        } else {
            file = std::fs::read_to_string(format!(
                "{}{}",
                home_dir_addr.unwrap().display(),
                "/.config/rlog/rlog.toml"
            ));
        }

        if file.is_ok() {
            match toml::from_str(&file.unwrap()) {
                Ok(value) => return value,
                Err(_) => {
                    eprintln!(
                        "The structure of your config file is not correct.\nUsing default config."
                    );
                    return config;
                }
            };
        } else {
            eprintln!("Can't open/read config file");
            return config;
        }
    } else {
        eprintln!("Can't find your home directory address.");
        return config;
    }
}
