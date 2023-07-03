use std::env;

// GH_TOKEN: an authentication token for github.com API requests.
// GH_EDITOR: the editor tool to use for authoring text.
// GH_BROWSER: the web browser to use for opening links.
// GH_DEBUG: set to a truthy value to enable verbose output on standard error.
const GH_TOKEN_KEY: &str = "GH_TOKEN";
const GH_EDITOR_KEY: &str = "GH_EDITOR";
const GH_EDITOR_KEY_FALLBACK: &str = "EDITOR";
const GH_BROWSER: &str = "GH_BROWSER";
const GH_BROWSER_FALLBAK: &str = "BROWSER";
const GH_DEBUG: &str = "GH_DEBUG";

/*
 * There are optimal ways to do this -> read value from given OS/variables
 * We do brute force tho
 */
fn read_key(key: &str) -> String {
    let mut vars = env::vars_os();
    let tup = vars.find(|(k, _)| k.to_str().unwrap().eq(key));
    let env_val = match tup {
        Some((_, v)) => v.into_string().unwrap(),
        None => String::new(),
    };
    env_val
}

pub struct Config {
    // required: throws error when empty
    pub gh_token: String,
    // optional
    pub gh_editor: String,
    pub gh_browser: String,
    // false by default
    pub gh_debug: bool,
}

impl Config {
    // setup default values
    pub fn default() -> Config {
        Config {
            gh_token: String::new(),
            gh_editor: String::new(),
            gh_browser: String::new(),
            gh_debug: false,
        }
    }

    fn read_token(&mut self) {
        let token_val = read_key(GH_TOKEN_KEY);
        if token_val.is_empty() {
            panic!("GH_TOKEN_KEY is empty, set it on your terminal env");
        }
        self.gh_token = token_val;
    }

    fn read_editor(&mut self) {
        let editor_val = read_key(GH_EDITOR_KEY);
        if !editor_val.is_empty() {
            self.gh_editor = editor_val;
            return;
        }
        let editor_val = read_key(GH_EDITOR_KEY_FALLBACK);
        if !editor_val.is_empty() {
            self.gh_editor = editor_val;
            return;
        }
        panic!(
            "Git editor unknown, set {} or  {} to your terminal env",
            GH_EDITOR_KEY, GH_EDITOR_KEY_FALLBACK
        );
    }

    fn read_browser(&mut self) {
        let browser_val = read_key(GH_BROWSER);
        if !browser_val.is_empty() {
            self.gh_browser = browser_val;
            return;
        }
        let browser_val = read_key(GH_BROWSER_FALLBAK);
        if !browser_val.is_empty() {
            self.gh_browser = browser_val;
            return;
        }
        panic!(
            "This program need to know browser to open github with, set {} or  {} to your terminal env",
            GH_BROWSER, GH_BROWSER_FALLBAK
        );
    }

    fn read_debug(&mut self) {
        let debug_flag_val = read_key(GH_DEBUG);
        match debug_flag_val.parse() {
            Ok(true) => self.gh_debug = true,
            _ => (),
        }
    }

    pub fn read(&mut self) {
        self.read_token();
        self.read_editor();
        self.read_browser();
        self.read_debug();
    }
}

pub struct ConfigReader {}

// Use ConfigReader::get() to read env variables
impl ConfigReader {
    pub fn get() -> Config {
        let mut c = Config::default();
        c.read();
        c
    }
}
