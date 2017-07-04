#[macro_use]
extern crate serde_derive;
extern crate serde;

pub const CONFIG_PATH: &'static str = "/etc/blochs/server.toml";

pub mod io {
    use std::path::Path;
    use std::fs::{create_dir, File};
    use std::io::Error;
    use std::io::prelude::*;

    pub fn ensure_dir_exists(path: &Path) {
        if !path.exists() {
            match create_dir(path) {
                Err(err) => panic!("Could not create directory under {}: {}", path.display(), err),
                Ok(_) => {}
            };
        }
    }

    pub fn get_file_content(path: &Path) -> String {
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

    pub fn rewrite_file(path: &Path, new_content: &String) -> Result<(), Error> {
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(reason) => panic!("Could not open file {:?}: {}", path.display(), reason),
        };

        return file.write_all(new_content.as_bytes());
    }
}

pub mod config {
    extern crate toml;

    use std::path::Path;
    use std::io::Error;


    #[derive(Serialize, Deserialize)]
    pub struct ServerConfig {
        pub data_dir: Option<String>,
        pub listening_port: Option<u64>,
    }

    pub fn load_server_config() -> ServerConfig {
        let file_path = Path::new(super::CONFIG_PATH);
        let dir_path = file_path.parent().unwrap();
        super::io::ensure_dir_exists(&dir_path);
        let actual_content = super::io::get_file_content(&file_path);
        let server_instance: ServerConfig = toml::from_str(&actual_content).unwrap();
        return server_instance;
    }

    pub fn store_server_config(values: &ServerConfig) -> Result<String, Error>  {
        let file_path = Path::new(super::CONFIG_PATH);
        let new_config_content = toml::to_string(&values).unwrap();
        return match super::io::rewrite_file(&file_path, &new_config_content) {
            Err(e) => Err(e),
            Ok(_) => Ok(new_config_content),
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
