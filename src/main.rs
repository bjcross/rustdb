#[macro_use]
extern crate clap;
use clap::App;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
struct Database {
    name: String,
    tables: HashMap<String, Table>,
    initialized: bool,
}

struct Table {
    name: String,
    columns: Vec<String>,
    table: HashMap<String, Vec<String>>,
}
impl Database {

}

impl Table {

}
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let input = matches.value_of("INPUT").unwrap();
    println!("Input file location is: {}",input);
    let port = matches.value_of("port").unwrap();
    println!("Port is {}", port);
    let port_int = u32::from_str_radix(port, 10).unwrap();
    println!("Port_int is {}", port_int);
    let createdb: bool = matches.is_present("createdb");
    let mut database: Database = if createdb == true {
        let dbname =  matches.value_of("createdb").unwrap();
        println!("Create db present is {}", dbname);
        Database {
            name: String::from(dbname),
            tables: HashMap::new(),
            initialized: false,
        }
    }
    else {
        Database {
            name: String::from("temptemp"),
            tables: HashMap::new(),
            initialized: false,
        }
    };
}
