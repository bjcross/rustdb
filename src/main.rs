#[macro_use]
extern crate clap;
use clap::App;
use std::error::Error;
use serde::{Serialize, Deserialize};
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let input = matches.value_of("INPUT").unwrap();
    println!("Input file location is: {}",input);
    let createdb: bool =  matches.is_present("createdb");
    println!("Create db present is {}", createdb);
    let port = matches.value_of("port").unwrap();
    println!("Port is {}", port);
    let port_int = u32::from_str_radix(port, 10).unwrap();
    println!("Port_int is {}", port_int);
}
