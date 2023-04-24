use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    println!("Magical Storage System Running!");
    match load_setver_config_from_file() {
        Ok(config) => {
            println!("load config!");
            println!("{}", config.host)
        }
        Err(err) => {
            println!("fail to load config, {}", err)
        }
    }
}

#[derive(Deserialize, Serialize)]
struct ServerConfig {
    host: String,
}

fn load_setver_config_from_file() -> Result<ServerConfig, String> {
    let home_dir = match env::var("HOME") {
        Ok(home) => PathBuf::from(home),
        Err(_) => panic!("Failed to get the user's home directory."),
    };
    let msconfig_path = home_dir.join(".msconfig");

    let config_file = File::open(msconfig_path.as_path());
    match config_file {
        Ok(file) => {
            let server_config: ServerConfig = serde_yaml::from_reader(file).unwrap();
            Result::Ok(server_config)
        }
        Err(_) => {
            println!("default config set up start.");
            let file = File::create(msconfig_path.as_path()).unwrap();
            let default_config = ServerConfig {
                host: String::from("ms.usaken.org"),
            };
            match serde_yaml::to_writer(file, &default_config) {
                Ok(_) => Result::Ok(default_config),
                Err(_) => Err("fail to write default config file".to_string()),
            }
        }
    }
}
