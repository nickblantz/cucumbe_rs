#[macro_use]
extern crate serde_derive;

pub mod parser;

use parser::ast::*;
use parser::dialect::Dialect;
use parser::io::GherkinBuffer;
use parser::parser::GherkinParser;
use std::fs::{self, File};
use parser::io::*;


fn main() {
    let dialect = Dialect::new(&"en");
    let file_name: &str = "features/my_first.feature";
    let lang_file: File = File::open(file_name).expect("error!!!");
    //fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let gherkin_buffer: GherkinBuffer<File> = GherkinBuffer::new(lang_file);
    println!("{:?}", &dialect);
    let _parser = GherkinParser::create(gherkin_buffer, dialect);
}
