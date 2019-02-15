extern crate serde_yaml;

mod tag;

use serde_yaml::Value;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(path) => path,
        None => {
            eprintln!("no file specified");
            process::exit(1);
        }
    };

    println!("reading from: {}", filename);

    let file = match File::open(filename) {
        Ok(f) => f,
        Err(reason) => {
            eprintln!("could not open {} due to: {}", filename, reason);
            process::exit(2);
        }
    };

    let systems: Value = match serde_yaml::from_reader(&file) {
        Ok(value) => value,
        Err(reason) => {
            eprintln!("could not parse {} due to: {}", filename, reason);
            process::exit(1);
        }
    };

    println!("{:?}", systems);
}
