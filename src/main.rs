#[macro_use]
extern crate clap;
use clap::App;
use std::error::Error;
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let input = matches.value_of("INPUT").unwrap();
    println!("Hello, world!");
}
