#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod config {
    #[derive(Serialize, Deserialize)]
    pub struct ServerConfig {
        pub data_dir: Option<String>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
