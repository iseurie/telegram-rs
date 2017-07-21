#[macro_use]
extern crate error_chain;

extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

mod error;
mod parser;
mod generator;

use std::fs::File;
use std::path::Path;
use std::io::Read;

pub fn translate(input_filename: &str, output_filename: &Path) -> error::Result<()> {
    let mut f = File::open(input_filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let s: parser::Schema = s.parse()?;

    generator::generate(output_filename, s)?;

    Ok(())
}
