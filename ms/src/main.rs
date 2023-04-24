mod load_server_config;

use load_server_config::load_setver_config_from_file;

fn main() {
    println!("Magical Storage System Running!");
    match load_setver_config_from_file() {
        Ok(config) => {
            println!("load config!");
            println!("{}", config.host);
        }
        Err(err) => {
            println!("fail to load config, {}", err);
        }
    }
}
