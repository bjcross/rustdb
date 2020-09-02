#[macro_use]
extern crate clap;
use clap::App;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Database {
    name: String,
    tables: HashMap<String, Table>,
    initialized: bool,
}

#[derive(Serialize, Deserialize)]
struct Table {
    name: String,
    columns: Vec<String>,
    table: HashMap<String, Vec<String>>
}

impl Database {
    fn add_table(&mut self, name: String, cols: Vec<String>) {
        let mut table = Table::new(name.clone(), cols);
        self.tables.insert(name,table);
        if self.initialized == false {
            self.initialized = true;
        }
    }
    fn get_table(&mut self, name: String) -> &Table {
        let mut table: &Table = self.tables.get(&name).unwrap();
        table
    }
}

impl Table {
    pub fn new(name: String, cols: Vec<String>) -> Table {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        Table { name: name, columns: cols, table: map }
    }
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
        let mut file = File::create(input).unwrap();
        let retvar = Database {
            name: String::from(dbname),
            tables: HashMap::new(),
            initialized: false,
        };
        let serialized = serde_json::to_string(&retvar).unwrap();
        file.write(serialized.as_bytes()).unwrap();
        retvar
    }
    else {
        let mut file = File::open(input).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let retvar: Database = serde_json::from_str(&contents).unwrap();
        retvar
    };
}
