#[macro_use]
extern crate serde_derive;

extern crate serde;

extern crate toml;
extern crate clap;

use clap::{App, Arg, ArgMatches};
use std::path::Path;
use std::fs::{create_dir, File};
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Config {
    data_dir: Option<String>,
}

const CONFIG_PATH: &'static str = "/etc/blochs/";
const CONFIG_FILE_NAME: &'static str = "server.toml";
const DEFAULT_DATA_DIR: &'static str = "/var/lib/blochs/";

fn main() {
    let options = get_provided_options();

    let config_path = Path::new(CONFIG_PATH);
    ensure_dir_exists(&config_path);

    let config_file_path = config_path.join(CONFIG_FILE_NAME);

    let actual_content = get_file_content(&config_file_path);

    let mut config_values: Config = toml::from_str(&actual_content).unwrap();

    let config_or_default_data_dir = config_values.data_dir.unwrap_or(DEFAULT_DATA_DIR.to_string());

    let new_data_dir = options.value_of("data.dir").unwrap_or(&config_or_default_data_dir);

    config_values.data_dir = Some(new_data_dir.to_string());

    let new_config_content = toml::to_string(&config_values).unwrap();


    let mut write_config_file = match File::create(config_file_path.as_path()) {
        Ok(file) => file,
        Err(err) => panic!("Could not open config file {:?}: {}", config_file_path.display(), err),
    };

    match write_config_file.write_all(new_config_content.as_bytes()) {
        Ok(_) => println!("New config saved at {}:\n\n{}", config_file_path.display(), new_config_content),
        Err(why) => panic!("Could not write config file {:?}: {}", config_file_path.display(), why),
    };
}

fn get_file_content(path: &Path) -> String {
    let mut content = String::new();
    if path.exists() {
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => panic!("Could not open file {:?}: {}", path.display(), err),
        };

        match file.read_to_string(&mut content) {
            Ok(_) => {},
            Err(err) => panic!("Could not read file {:?}: {}", path.display(), err),
        };
    }
    return content;
}

fn ensure_dir_exists(path: &Path) {
    if !path.exists() {
        match create_dir(path) {
            Err(err) => panic!("Could not create directory under {}: {}", path.display(), err),
            Ok(_) => {}
        };
    }
}

fn get_provided_options<'a>() -> ArgMatches<'a> {
    return App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("data.dir")
            .long("data-dir")
            .value_name("DIR")
            .help(&format!("Sets where database data will be stored (default {})", DEFAULT_DATA_DIR))
        ).get_matches();
}
