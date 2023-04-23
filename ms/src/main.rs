use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::io::Write;

fn main() {
    println!("Magical Storage System Running!");
    if load_setver_config_from_file() {
        println!("config loaded");
    } else {
        println!("config load failed");
    }
}

fn load_setver_config_from_file() -> bool{
    let home_dir = match env::var("HOME") {
        Ok(home) => PathBuf::from(home),
        Err(_) => panic!("Failed to get the user's home directory."),
    };
    let msconfig_path  = home_dir.join(".msconfig");

    let config_file: std::io::Result<File> = File::open(msconfig_path.as_path());
    
    if config_file.is_err() {
        let mut file = File::create(msconfig_path.as_path()).unwrap();
        file.write(b"host: ms.usaken.org\n").unwrap();
    }
    return config_file.is_ok()
}
