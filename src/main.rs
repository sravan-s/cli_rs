mod config;

pub fn main() {
    let c = config::ConfigReader::get();
    println!("{}", c.gh_token);
}
