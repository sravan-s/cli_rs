use std::env;

const GH_TOKEN: &str = "GH_TOKEN";

pub struct Config {
    pub gh_token: String,
}

impl Config {
    pub fn init(&mut self) {
        let vars = env::vars_os();
        let gh_token = vars.into_iter().find(|(key, _)| key.eq(GH_TOKEN));
        let value_optn: String = match gh_token {
            Some((_token, value)) => value.to_str().unwrap().to_string(),
            None => panic!("No GH_TOKEN in env. "),
        };

        if value_optn.is_empty() {
            panic!("GH_TOKEN is empty, add it to your ENV");
        }
        self.gh_token = value_optn;
    }
}

pub struct ConfigReader {}

// Use ConfigReader::get() to read env variables
impl ConfigReader {
    pub fn get() -> Config {
        let mut c = Config {
            gh_token: String::new(),
        };
        c.init();
        c
    }
}
