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


pub fn translate_from_json_file<I, O>(input_filename: I, output_filename: O) -> error::Result<()>
    where I: AsRef<Path>,
          O: AsRef<Path>,
{
    let f = File::open(input_filename)?;
    let s: parser::Schema = serde_json::from_reader(f)?;
    generator::generate(output_filename, &s)?;

    Ok(())
}
