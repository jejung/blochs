extern crate clap;
extern crate libblochs;

use clap::{App, Arg, ArgMatches};
use libblochs::config::{ServerConfig, load_server_config, store_server_config};
use libblochs::CONFIG_PATH;

const DEFAULT_DATA_DIR: &'static str = "/var/lib/blochs/";

fn main() {
    let options = get_provided_options();

    let mut config_values = load_server_config();

    set_options(&mut config_values, &options);

    match store_server_config(&config_values) {
        Ok(content) => println!("New config saved at {}:\n\n{}", CONFIG_PATH, content),
        Err(why) => panic!("Could not write config file {:?}: {}", CONFIG_PATH, why),
    };
}

fn set_options(config_values: &mut ServerConfig, options: &ArgMatches) {
    let config_data_dir = config_values.data_dir.take();

    let config_or_default_data_dir = config_data_dir.unwrap_or(DEFAULT_DATA_DIR.to_string());
    let new_data_dir = options.value_of("data.dir").unwrap_or(&config_or_default_data_dir);

    config_values.data_dir = Some(new_data_dir.to_string());
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
