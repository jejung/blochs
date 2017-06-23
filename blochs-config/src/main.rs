extern crate toml;
extern crate clap;
extern crate libblochs;

use clap::{App, Arg, ArgMatches};
use std::path::Path;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::io::Result;
use libblochs::config::ServerConfig;

const CONFIG_PATH: &'static str = "/etc/blochs/";
const CONFIG_FILE_NAME: &'static str = "server.toml";
const DEFAULT_DATA_DIR: &'static str = "/var/lib/blochs/";

fn main() {
    let options = get_provided_options();

    let config_path = Path::new(CONFIG_PATH);
    ensure_dir_exists(&config_path);

    let config_file_path = config_path.join(CONFIG_FILE_NAME);
    let actual_content = get_file_content(&config_file_path);

    let mut config_values: ServerConfig = toml::from_str(&actual_content).unwrap();
    set_options(&mut config_values, &options);

    let new_config_content = toml::to_string(&config_values).unwrap();
    match rewrite_file(&config_file_path, &new_config_content) {
        Ok(_) => println!("New config saved at {}:\n\n{}", config_file_path.display(), new_config_content),
        Err(why) => panic!("Could not write config file {:?}: {}", config_file_path.display(), why),
    };
}

fn set_options(config_values: &mut ServerConfig, options: &ArgMatches) {
    let config_data_dir = config_values.data_dir.take();

    let config_or_default_data_dir = config_data_dir.unwrap_or(DEFAULT_DATA_DIR.to_string());
    let new_data_dir = options.value_of("data.dir").unwrap_or(&config_or_default_data_dir);

    config_values.data_dir = Some(new_data_dir.to_string());
}

fn rewrite_file(path: &Path, new_content: &String) -> Result<()> {
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(err) => panic!("Could not open config file {:?}: {}", path.display(), err),
    };

    return file.write_all(new_content.as_bytes());
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
