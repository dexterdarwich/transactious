extern crate csv;
extern crate serde;
//#[macro_use]
extern crate serde_derive;

use std::process;

pub mod engine;
pub mod processor;
pub mod storage;

fn main() {
    if let Err(err) = processor::process() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
